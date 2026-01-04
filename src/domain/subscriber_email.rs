use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
       if s.validate_email() {
            Ok(Self(s))
       } else {
            Err(format!("{s} is not a valid email address"))
       }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_err;
    use fake::{Fake, faker::internet::en::SafeEmail, rand};

    use super::*;

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let name = "ursulasdomain.com".into();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let name = "@domain.com".into();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn a_white_space_only_name_is_rejectted() {
        let name = " ".to_string();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn valid_email_are_parsed_successfully() {
        let email:String = SafeEmail().fake();
        claims::assert_ok!(SubscriberEmail::parse(email));
    }

    #[derive(Clone, Debug)]
    struct ValidateEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidateEmailFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::rng();
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidateEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}