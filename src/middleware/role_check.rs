use crate::server::Role;

use super::{Middleware, Request};

#[derive(Default)]
pub struct RoleCheck {
    next: Option<Box<dyn Middleware>>,
}

impl Middleware for RoleCheck {
    fn handle(&mut self, request: &mut Request) -> bool {
        match (request.url.as_ref(), request.user.role) {
            ("/admin", Role::Admin) => {
                println!("Admin is allowed to enter the admin page!");
                return true;
            },
            ("/home", _) => {
                println!("User is allowed to enter the home page!");
                return true;
            }
            _ => {
                println!("Either the page does not exists or you do not have the rights to enter it!");
                return false;
            }
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Middleware>> {
        &mut self.next
    }
}