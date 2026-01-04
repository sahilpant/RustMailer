use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Deserialize, Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self,String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/','(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_charachters = s.chars().any(|s| forbidden_characters.contains(&s));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_charachters {
            Err(format!("{} is not a valid subscriber name.",s))
        } else {
            Ok(Self(s))
        }
        
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use super::*;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn a_white_space_only_name_is_rejectted() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }
}