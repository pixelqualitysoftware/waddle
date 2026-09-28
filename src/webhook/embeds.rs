use crate::global::random_footer;

use poise::serenity_prelude as serenity;
use serenity::ButtonStyle;
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};

pub fn create_push_embed(
    repo: &str,
    branch: &str,
    user: &str,
    commits: usize,
    commit_url: &str,
) -> (CreateEmbed, CreateActionRow) {
    let push_embed = CreateEmbed::new()
        .title(format!("New Push to {repo}"))
        .field("Branch", branch, true)
        .field("Pushed by", user, true)
        .field("Commits", commits.to_string(), true)
        .url(commit_url)
        .footer(random_footer());

    let button = CreateActionRow::Buttons(vec![url_button_maker("View Commit", commit_url)]);

    (push_embed, button)
}

pub fn create_issue_embed(
    repo: &str,
    user: &str,
    issue: &str,
    action: &str,
    issue_url: &str,
) -> (CreateEmbed, CreateActionRow) {
    let title = match action {
        "opened" => format!("New Issue Opened in {repo}"),
        "closed" => format!("Issue Closed in {repo}"),
        _ => format!("Issue Updated in {repo}"),
    };

    let issue_embed = CreateEmbed::new()
        .title(title)
        .field("Opened by", user, true)
        .field("Issue", issue, true)
        .footer(random_footer())
        .url(issue_url);

    let button = CreateActionRow::Buttons(vec![url_button_maker("View Issue", issue_url)]);

    (issue_embed, button)
}

pub fn create_comment_embed(
    user: &str,
    issue: &str,
    action: &str,
    url: &str,
    comment_url: &str,
    comment: &str,
) -> (CreateEmbed, CreateActionRow) {
    let title = match action {
        "created" => format!("New Comment on {issue}"),
        "edited" => format!("Comment Edited on {issue}"),
        "deleted" => format!("Comment Deleted on {issue}"),
        _ => format!("Comment Updated on {issue}"),
    };

    let issue_comment_embed = CreateEmbed::new()
        .title(title)
        .field("Opened by", user, true)
        .field("Issue", issue, true)
        .field("Comment", comment, false)
        .footer(random_footer())
        .url(url);

    let button = CreateActionRow::Buttons(vec![url_button_maker("View Comment", comment_url)]);

    (issue_comment_embed, button)
}

pub fn create_pr_embed(
    repo: &str,
    user: &str,
    pr_title: &str,
    action: &str,
    pr_url: &str,
    merged: bool,
    merged_by: &str,
) -> (CreateEmbed, CreateActionRow) {
    let title = match action {
        "opened" => format!("New Pull Request in {repo}"),
        "closed" if merged => format!("Pull Request Merged in {repo}"),
        "closed" => format!("Pull Request Closed in {repo}"),
        _ => format!("Pull Request Updated in {repo}"),
    };

    let button = CreateActionRow::Buttons(vec![url_button_maker("View PR", pr_url)]);

    if merged {
        let embed = CreateEmbed::new()
            .title(title)
            .field("Pull Request", pr_title, true)
            .field("Opened by", user, true)
            .field("Merged by", merged_by, true)
            .footer(random_footer())
            .url(pr_url);

        (embed, button)
    } else {
        let embed = CreateEmbed::new()
            .title(title)
            .field("Pull Request", pr_title, true)
            .field("Opened by", user, true)
            .footer(random_footer())
            .url(pr_url);

        (embed, button)
    }
}

pub fn _button_maker(text: &str, id: &str) -> CreateButton {
    CreateButton::new(id)
        .label(text)
        .style(ButtonStyle::Primary)
}

pub fn url_button_maker(text: &str, url: &str) -> CreateButton {
    CreateButton::new_link(url).label(text)
}
