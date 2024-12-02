use crate::manager::Account;
use rpassword::read_password;
use serde_json::to_writer_pretty;
use std::fs::create_dir_all;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// Retrieves a list of accounts stored in the `passwords.json` file.
///
/// This function reads the JSON file containing account information and deserializes
/// it into a `Vec<Account>`. If the file is empty, it returns an empty vector.
///
/// # Returns
///
/// Returns a `serde_json::Result<Vec<Account>>` which contains either:
/// - `Ok(Vec<Account>)` on success, where `Vec<Account>` is the list of accounts.
/// - `Err(serde_json::Error)` if the JSON data is malformed.
///
/// # Example
///
/// ```rust
/// let accounts = util::get_data().unwrap();
/// println!("{:?}", accounts);
/// ```
pub fn get_data() -> serde_json::Result<Vec<Account>> {
    let path = get_password_file_path();
    let file = File::open(&path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Could not read file");

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let accounts: Vec<Account> =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(accounts)
}

/// Appends an account to the `passwords.json` file.
///
/// This function adds a new account to the JSON file. If an account with the same label
/// already exists, it will print an error message and not save the new account.
///
/// # Arguments
///
/// * `account` - The account to be saved to the file.
///
/// # Example
///
/// ```rust
/// let new_account = Account {
///     label: String::from("example"),
///     username: String::from("John"),
///     email: String::from("JohnDoe@example.com"),
///     password: String::from("password"),
///     description: String::from("example account"),
/// };
/// util::append_to_file(new_account);
/// ```
///
/// # Errors
///
/// This function will panic if there are issues opening, writing, or serializing the file.
pub fn append_to_file(account: Account) {
    let mut accounts: Vec<Account> = match get_data() {
        Ok(accounts) => accounts,
        Err(e) => {
            println!("Error getting data: {}", e);
            return;
        }
    };
    let label = account.label.clone();

    if accounts.iter().any(|account| account.label == label) {
        println!(
            "Error: An entry with the label '{}' already exists. \
            Please use a different label",
            label
        );
        return;
    }

    accounts.push(account);
    save_to_file(&accounts);

    println!(
        "Success: The entry for '{}' has been saved successfully",
        label
    );
}

/// Saves all accounts to the `passwords.json` file.
///
/// This function overwrites the existing `passwords.json` file with the provided list
/// of accounts. It ensures that the file contains the latest account data.
///
/// # Arguments
///
/// * `accounts` - A reference to a `Vec<Account>` containing the accounts to be saved.
///
/// # Example
///
/// ```rust
/// util::save_to_file(&accounts);
/// ```
///
/// # Errors
///
/// This function will panic if there are issues opening, writing, or serializing the file.
pub fn save_to_file(accounts: &Vec<Account>) {
    let path = get_password_file_path();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Could not save to file");

    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &accounts).unwrap();
}

/// Prompts the user to input a string.
///
/// This function prints the provided message and waits for the user to type input.
/// It returns the input as a `String` after the user presses Enter.
///
/// # Arguments
///
/// * `message` - A reference to the prompt message that will be displayed to the user.
///
/// # Returns
///
/// A `String` containing the user’s input, with leading and trailing whitespace removed.
///
/// # Example
///
/// ```rust
/// let user_input = util::get_user_input("Enter your username: ");
/// println!("Username: {}", user_input);
/// ```
pub fn get_user_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Retrieves the path to the `passwords.json` file used by the password manager.
///
/// This function constructs the path to the `passwords.json` file located in the
/// `APPDATA` directory. It ensures that any missing parent directories are created,
/// and if the file does not exist, it is created.
///
/// # Returns
///
/// Returns a `PathBuf` representing the full path to the `passwords.json` file.
///
/// # Example
///
/// ```rust
/// let file_path = get_password_file_path();
/// println!("Password file path: {:?}", file_path);
/// ```
///
/// # Errors
///
/// This function will panic if the directories cannot be created or the file cannot
/// be created due to permission issues or other I/O errors.
pub fn get_password_file_path() -> PathBuf {
    let app_data_path = std::env::var("APPDATA").unwrap();
    let path = Path::new(&app_data_path).join("Password Manager/data/passwords.json");

    if let Some(parent) = path.parent() {
        if let Some(grandparent) = parent.parent() {
            create_dir_all(grandparent).expect("Could not create directory structure");
        }
        create_dir_all(parent).expect("Could not create data directory");
    }

    // Create the passwords.json file if it doesn't exist
    if !path.exists() {
        File::create(&path).expect("Could not create file");
    }

    path
}

/// Checks if the `passwords.json` file exists.
///
/// This function checks whether the `passwords.json` file exists on the user machine.
///
/// # Returns
///
/// Returns `true` if the file exists, otherwise returns `false`.
///
/// # Example
///
/// ```rust
/// if util::file_exists() {
///     println!("File exists!");
/// } else {
///     println!("File does not exist.");
/// }
/// ```
pub fn file_exists() -> bool {
    let path = get_password_file_path();
    path.exists()
}

/// Centers the given text within a 5-character width.
///
/// If the input text is shorter than 5 characters, it is padded with spaces on both sides
/// to make it 5 characters wide. If the text is already 5 characters or longer, it is returned as-is.
///
/// # Arguments
///
/// * `value` - The text to be centered.
///
/// # Returns
///
/// A `String` with the text centered within a 5-character width.
///
/// # Example
///
/// ```rust
/// let centered = util::center_align_text("abc");
/// println!("Centered: '{}'", centered); // Output: ' abc '
/// ```
pub fn center_align_text(value: &str) -> String {
    let value_len = value.len();
    if value_len >= 5 {
        return format!("{:<width$}", value, width = 5);
    }

    let padding_total = 5 - value_len;
    let left_padding = padding_total / 2;
    let right_padding = padding_total - left_padding;

    format!(
        "{}{}{}",
        " ".repeat(left_padding),
        value,
        " ".repeat(right_padding)
    )
}

/// Prompts the user to input and confirm a password.
///
/// This function repeatedly asks the user to input a password and confirm it.
/// If the two inputs match, it returns the password. If they don't match, it will
/// prompt the user again until the passwords match.
///
/// # Returns
///
/// The password entered by the user as a `String`.
///
/// # Example
///
/// ```rust
/// let password = util::get_password();
/// println!("Password: {}", password);
/// ```
pub fn get_password() -> String {
    loop {
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
    }
}
