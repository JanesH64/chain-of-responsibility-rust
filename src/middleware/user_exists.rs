use std::collections::HashMap;

use crate::server::{Server, User, Role};

use super::{into_next, Middleware, Request};

#[derive(Default)]
pub struct UserExists {
    next: Option<Box<dyn Middleware>>,
    server: Server
}

impl UserExists {
    pub fn new(next: impl Middleware + 'static) -> Self {
        let admin = User {
            username: "Hans_Peter".into(),
            password: "1234".into(),
            role: Role::Admin
        };

        let user = User {
            username: "ExistiertNicht".into(),
            password: "1234".into(),
            role: Role::User
        };
        
        let mut server = Server {
            users: HashMap::new()
        };
    
        server.users.insert(admin.username.to_owned(), admin);
        server.users.insert(user.username.to_owned(), user);
        
        Self {
            next: into_next(next),
            server: server
        }
    }
}

impl Middleware for UserExists {
    fn handle(&mut self, request: &mut Request) -> bool {
        if self.server.users.contains_key(&request.user.username) {
            println!("User exists!");
            return true;
        }
        else {
            println!("User not found!");
            return false;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Middleware>> {
        &mut self.next
    }
}