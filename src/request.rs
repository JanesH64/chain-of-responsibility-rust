use crate::server::User;

#[derive(Default)]
pub struct Request {
    pub user: User,
    pub url: String
}