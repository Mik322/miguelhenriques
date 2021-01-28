extern crate backend;
use backend::models::user;

use std::io;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() -> io::Result<()> {
    println!("Username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    trim_newline(&mut username);

    println!("Password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    trim_newline(&mut password);

    let conn = backend::create_pool().get().unwrap();

    let user_data = user::UserLoginData {
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    match user::User::create_user(&conn, user_data) {
        Ok(_) => println!("User Created"),
        Err(str) => println!("{}", str),
    };

    Ok(())
}
