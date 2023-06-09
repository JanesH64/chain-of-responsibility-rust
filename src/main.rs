

use middleware::{UserExists, RoleCheck, Middleware};
use request::Request;
use server::{User};

mod middleware;
mod request;
mod server;


fn main() {
    let role_check = RoleCheck::default();
    let mut user_exists = UserExists::new(role_check);

    let administrative_user = User {
        username: "Hans_Peter".into(),
        password: "1234".into(),
        role: server::Role::Admin
    };

    let user = User {
        username: "Hans_Juergen".into(),
        password: "1234".into(),
        role: server::Role::User
    };

    let mut admin_request = Request {
        user: administrative_user,
        url: "/admin".into()
    };

    let mut user_request = Request {
        user: user,
        url: "/admin".into()
    };

    println!("Admin request: ");
    if user_exists.execute(&mut admin_request) {
        println!("Request was successful!");
    }
    else {
        println!("Request failed!");
    }
    println!("");
    println!("User request: ");
    if user_exists.execute(&mut user_request) {
        println!("Request was successful!");
    }
    else {
        println!("Request failed!");
    }
}