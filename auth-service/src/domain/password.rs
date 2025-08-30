#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid password", s))
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

// AsRef trait allows us to get a &str from a Password
impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[test]
    fn empty_password_is_invalid() {
        let password = "".to_string();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn short_password_is_invalid() {
        let password = "short".to_string();
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..20).fake_with_rng(g);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool{
        Password::parse(valid_password.0).is_ok()
    }
}