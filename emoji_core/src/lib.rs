pub fn lookup_emoji(key: &str) -> Option<&'static str> {
    emojic::parse_alias(format!(":{}:", key).as_str()).map(|emoji| emoji.grapheme)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_emoji() {
        // Test that the lookup_emoji function returns the correct emoji
        // for a given alias.
        assert_eq!(lookup_emoji("smile"), Some("😄"));
        assert_eq!(lookup_emoji("smiley"), Some("😃"));
        assert_eq!(lookup_emoji("grinning"), Some("😀"));
        assert_eq!(lookup_emoji("grin"), Some("😁"));
        assert_eq!(lookup_emoji("joy"), Some("😂"));
        assert_eq!(lookup_emoji("rofl"), Some("🤣"));
        assert_eq!(lookup_emoji("relaxed"), Some("☺️"));
        assert_eq!(lookup_emoji("blush"), Some("😊"));

        // Tests for aliases that have multiple emoji.
        assert_eq!(lookup_emoji("heart"), Some("❤️"));
        assert_eq!(lookup_emoji("thumbsup"), Some("👍"));
        assert_eq!(lookup_emoji("thumbsdown"), Some("👎"));

        // Test that the lookup_emoji function returns None for an
        // invalid alias.
        assert_eq!(lookup_emoji("invalid"), None);
    }
}
