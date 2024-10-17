use std::borrow::Cow;
use crate::util;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use textwrap::wrap;

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
        let wrap_label = wrap(&self.label, 28);
        let wrap_username = wrap(&self.username, 28);
        let wrap_email = wrap(&self.email, 28);
        let wrap_password = wrap(&self.password, 28);
        let wrap_description = wrap(&self.description, 28);

        Self::format(6, "Label".to_string(), wrap_label);
        Self::format(3, "Username".to_string(), wrap_username);
        Self::format(6, "Email".to_string(), wrap_email);
        Self::format(3, "Password".to_string(), wrap_password);
        Self::format(0, "Description".to_string(), wrap_description);
    }

    fn format(empty_spaces: usize, attribute_name: String, wrap_description: Vec<Cow<str>>) {
        let data_width: usize = 31;

        for (index, line) in wrap_description.into_iter().enumerate() {
            if index == 0 {
                println!("│   {}: {:width$}│", attribute_name, line, width = data_width + empty_spaces);
            } else {
                println!("│                {:width$}│", line, width = data_width);
            }
        }
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

pub fn display_accounts(simplify: bool) {
    let accounts = util::get_data().unwrap();

    for (index, account) in accounts.iter().enumerate() {
        let centered = util::center_align_text(&*(index + 1).to_string());

        if !simplify {
            println!("┌{}< {} >{}┐", "─".repeat(19), centered, "─".repeat(19));
            account.display();
            println!("└{}┘", "─".repeat(47));
        } else {
            println!("{}. {}", index + 1, account.label);
        }
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