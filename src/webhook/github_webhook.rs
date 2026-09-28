use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::sync::mpsc;

use crate::webhook::embeds::{
    create_comment_embed, create_issue_embed, create_pr_embed, create_push_embed,
};
use poise::serenity_prelude as serenity;

use crate::types::Limit;
use crate::types::discord_limits::{FIELD, TITLE};

pub fn format_event(event: &Event) -> Option<(serenity::CreateEmbed, serenity::CreateActionRow)> {
    let user = event.payload["sender"]["login"]
        .as_str()
        .unwrap_or("someone")
        .limit(TITLE);

    let repo = event.payload["repository"]["full_name"]
        .as_str()
        .unwrap_or("unknown repo")
        .limit(TITLE);

    let repo_url = event.payload["repository"]["html_url"]
        .as_str()
        .unwrap_or("couldn't get repo url");

    let action = event.payload["action"].as_str().unwrap_or("updated");

    match event.name.as_str() {
        "push" => {
            let ref_str = event.payload["ref"].as_str().unwrap_or("refs/heads/main");
            let branch = ref_str
                .strip_prefix("refs/heads/")
                .unwrap_or(ref_str)
                .limit(TITLE);

            let commits = event.payload["commits"]
                .as_array()
                .map_or(0, std::vec::Vec::len);

            let sha = event.payload["after"]
                .as_str()
                .unwrap_or("couldn't get sha");

            let commit_url = format!("{repo_url}/commit/{sha}");

            Some(create_push_embed(
                &repo,
                &branch,
                &user,
                commits,
                &commit_url,
            ))
        }

        "issues" => {
            let url = event.payload["issue"]["html_url"]
                .as_str()
                .unwrap_or(repo_url);

            let issue_title = event.payload["issue"]["title"]
                .as_str()
                .unwrap_or("couldn't get title")
                .limit(TITLE);

            Some(create_issue_embed(&repo, &user, &issue_title, action, url))
        }

        "issue_comment" => {
            let url = event.payload["issue"]["html_url"]
                .as_str()
                .unwrap_or(repo_url);

            let comment_url = event.payload["comment"]["html_url"]
                .as_str()
                .unwrap_or("failed to get comment URL");

            let issue_title = event.payload["issue"]["title"]
                .as_str()
                .unwrap_or("couldn't get title")
                .limit(TITLE);

            let comment = event.payload["comment"]["body"]
                .as_str()
                .unwrap_or("couldn't fetch comment")
                .limit(FIELD);

            Some(create_comment_embed(
                &user,
                &issue_title,
                action,
                url,
                comment_url,
                &comment,
            ))
        }

        "pull_request" => {
            let pr = &event.payload["pull_request"];

            let pr_title = pr["title"]
                .as_str()
                .unwrap_or("couldn't get title")
                .limit(TITLE);

            let pr_url = pr["html_url"].as_str().unwrap_or("failed");
            let merged = pr["merged"].as_bool().unwrap_or(false);
            let merged_by = pr["merged_by"]["login"]
                .as_str()
                .unwrap_or("unknown")
                .limit(TITLE);

            Some(create_pr_embed(
                &repo, &user, &pr_title, action, pr_url, merged, &merged_by,
            ))
        }
        _ => None,
    }
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub delivery_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Clone)]
struct AppState {
    secret: String,
    sender: mpsc::Sender<Event>,
}

pub async fn start(
    address: String,
    secret: String,
) -> Result<mpsc::Receiver<Event>, std::io::Error> {
    let (sender, receiver) = mpsc::channel(64);
    let state = AppState { secret, sender };
    let app = Router::new()
        .route("/github", post(receive))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tokio::spawn(async move {
        if let Err(error) = axum::serve(listener, app).await {
            eprintln!("GitHub webhook listener stopped: {error}");
        }
    });
    Ok(receiver)
}

async fn receive(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(signature) = headers
        .get("x-hub-signature-256")
        .and_then(|v| v.to_str().ok())
    else {
        return (StatusCode::UNAUTHORIZED, "missing GitHub signature");
    };
    if !verify_signature(&state.secret, &body, signature) {
        return (StatusCode::UNAUTHORIZED, "invalid GitHub signature");
    }
    let Ok(payload) = serde_json::from_slice(&body) else {
        return (StatusCode::BAD_REQUEST, "request body is not valid");
    };
    let event = Event {
        name: headers
            .get("x-github-event")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_owned(),
        delivery_id: headers
            .get("x-github-delivery")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned),
        payload,
    };
    match state.sender.send(event).await {
        Ok(()) => (StatusCode::NO_CONTENT, ""),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "webhook receiver unavailable",
        ),
    }
}

fn verify_signature(secret: &str, body: &[u8], signature: &str) -> bool {
    let Some(value) = signature.strip_prefix("sha256=") else {
        return false;
    };
    let Ok(expected) = hex::decode(value) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(body);
    mac.verify_slice(&expected).is_ok()
}
