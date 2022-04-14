#[derive(Debug)]
pub struct User<'a> {
    username: &'a str,
    email: &'a str,
    pub nickname: &'a str,
    sign_in_count: u64,
    active: bool,
}

impl<'a> User<'a> {
    pub fn new(username: &'a str, email: &'a str, sign_in_count: u64, active: bool) -> Self {
        User {
            username,
            email,
            nickname: "admin",
            sign_in_count,
            active,
        }
    }

    pub fn updateUsername(&mut self, username: &'a str) {
        self.username = username;
    }
}
