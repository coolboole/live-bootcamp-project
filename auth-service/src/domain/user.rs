pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) requires_2fa: bool,
}

impl User {
    pub(crate) fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}