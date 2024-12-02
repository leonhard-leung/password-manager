use crate::util;
use crate::util::get_data;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use textwrap::wrap;

/// Represents an account stored in the password manager.
///
/// This struct holds the account's label, username, email, password, and description.
///
/// # Fields
///
/// * `label` - A short identifier for the account (e.g., "Facebook", "Email").
/// * `username` - The username associated with the account.
/// * `email` - The email address associated with the account.
/// * `password` - The password for the account.
/// * `description` - A description or notes about the account.
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

        // Print each field of the account in a formatted manner
        Self::format(6, "Label".to_string(), wrap_label);
        Self::format(3, "Username".to_string(), wrap_username);
        Self::format(6, "Email".to_string(), wrap_email);
        Self::format(3, "Password".to_string(), wrap_password);
        Self::format(0, "Description".to_string(), wrap_description);
    }

    /// Formats and prints each attribute of the account with specific indentation and wrapping.
    ///
    /// This function ensures that the output is properly aligned and wrapped.
    ///
    /// # Arguments
    ///
    /// * `empty_spaces` - The number of spaces to pad before printing the field name.
    /// * `attribute_name` - The name of the attribute being displayed (e.g., "Label").
    /// * `wrap_description` - The wrapped lines of the field's value.
    ///
    /// # Example
    ///
    /// ```rust
    /// Account::format(4, "Label".to_string(), wrapped_label);
    /// ```
    fn format(empty_spaces: usize, attribute_name: String, wrap_description: Vec<Cow<str>>) {
        let data_width: usize = 31;

        for (index, line) in wrap_description.into_iter().enumerate() {
            if index == 0 {
                println!(
                    "│   {}: {:width$}│",
                    attribute_name,
                    line,
                    width = data_width + empty_spaces
                );
            } else {
                println!("│                {:width$}│", line, width = data_width);
            }
        }
    }
}

/// Prompts the user to input information for a new account and saves it.
///
/// This function collects user input for the account's label, username, email,
/// password, and description. After collecting the necessary data, it creates
/// a new `Account` and saves it to the `passwords.json` file using `util::save_to_file`.
///
/// # Example
///
/// ```rust
/// util::add();
/// ```
pub fn add() {
    let label = util::get_user_input("Label: ");
    let username = util::get_user_input("Username: ");
    let email = util::get_user_input("Email: ");
    let password = util::get_password();
    let description = util::get_user_input("Description: ");

    let new_account = Account {
        label,
        username,
        email,
        password,
        description,
    };

    util::append_to_file(new_account);
}

/// Removes an existing account from the application.
///
/// This function reads the `passwords.json` file and checks if the provided label
/// matches an existing account. If a match is found, the account is removed from
/// the file and the updated data is saved. If no match is found, an error message
/// is displayed.
///
/// # Arguments
///
/// * `label` - the name of the account to be removed.
///
/// # Example
/// ```rust
/// let label = String::from("Sample Account");
/// util::remove(label);
/// ```
///
/// # Errors
///
/// This function will panic if the file cannot be written to during the save operation.
pub fn remove(label: String) {
    let mut accounts: Vec<Account> = match get_data() {
        Ok(accounts) => accounts,
        Err(e) => {
            println!("Error getting data: {}", e);
            return;
        }
    };

    if let Some(index) = accounts.iter().position(|account| account.label == label) {
        accounts.remove(index);
        util::save_to_file(&accounts);
        println!("Success: The entry '{}' has been removed", label);
    } else {
        println!(
            "Error: An entry with the label '{}' cannot be found. \
            Please check the label and try again",
            label
        );
    }
}

/// Displays a list of all accounts in the password manager.
///
/// This function prints all stored accounts. If `simplify` is set to `true`,
/// it only displays the account labels and their index. Otherwise, it displays
/// the full details of each account, including label, username, email, password,
/// and description, in a formatted table-like structure.
///
/// # Arguments
///
/// * `simplify` - A flag indicating whether to display only the account labels
///   (`true` for simplified, `false` for detailed view).
///
/// # Example
///
/// ```rust
/// util::display_accounts(false);  // Display full account details.
/// util::display_accounts(true);   // Display only account labels.
pub fn display_accounts(simplify: bool) {
    let accounts = util::get_data().unwrap();

    for (index, account) in accounts.iter().enumerate() {
        let centered = util::center_align_text(&(index + 1).to_string());

        if !simplify {
            println!("┌{}< {} >{}┐", "─".repeat(19), centered, "─".repeat(19));
            account.display();
            println!("└{}┘", "─".repeat(47));
        } else {
            println!("{}. {}", index + 1, account.label);
        }
    }
}
