pub trait Limit {
    fn limit(&self, max: usize) -> String;
}

impl Limit for str {
    fn limit(&self, max: usize) -> String {
        if self.chars().count() <= max {
            return self.to_string();
        }

        let target_chars = max.saturating_sub(1);
        let end_byte = self
            .char_indices()
            .nth(target_chars)
            .map_or(self.len(), |(idx, _)| idx);

        format!("{}…", &self[..end_byte])
    }
}

pub mod discord_limits {
    pub const TITLE: usize = 256;
    pub const FIELD: usize = 1024;
}
