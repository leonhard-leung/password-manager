use std::io;
use std::io::Write;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use crate::util;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub label: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub description: String,
}

impl Account {
    pub fn display(&self) {
        println!("Label: {}", self.label);
        println!("Email: {}", self.email);
        println!("Username: {}", self.username);
        println!("Password: {}", self.password);
        println!("Description: {}", self.description);
    }
}

pub fn add() {
    let label = util::get_user_input("Label: ");
    let username = util::get_user_input("Username: ");
    let email = util::get_user_input("Email: ");
    let password = obtain_password();
    let description = util::get_user_input("Description: ");

    let new_account = Account {
        label,
        username,
        email,
        password,
        description,
    };

    util::save_to_file(new_account);
}

pub fn display_accounts() {
    let accounts = util::get_data().unwrap();

    for account in &accounts {
        println!("{:#?}", account.display())
    }
}

fn obtain_password() -> String {
    let password = loop {
        print!("Enter Password: ");
        io::stdout().flush().unwrap();
        let password = read_password().unwrap();

        print!("Confirm Password: ");
        io::stdout().flush().unwrap();
        let confirm_password = read_password().unwrap();

        if password.eq(&confirm_password) {
            break password;
        } else {
            println!("Password Mismatch. Please try again.");
        }
    };
    password
}