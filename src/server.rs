use std::collections::HashMap;

#[derive(Default)]
pub struct Server {
    pub users: HashMap<String, User>
}

#[derive(Default)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role
}


#[derive(Default, Clone, Copy)]
pub enum Role {
    #[default]
    User,
    Admin
}