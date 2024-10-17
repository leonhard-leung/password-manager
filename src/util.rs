use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::fs::create_dir_all;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use serde_json::to_writer_pretty;
use crate::manager::Account;

pub fn get_data() -> serde_json::Result<Vec<Account>> {
    let path = get_password_file_path();
    let file = File::open(&path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Could not read file");

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let accounts: Vec<Account> = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(accounts)
}

pub fn save_to_file(account: Account) {
    let mut accounts: Vec<Account> = get_data().unwrap();
    let label = account.label.clone();

    if accounts.iter().any(|account| account.label == label) {
        println!("Error: An entry with the label '{}' already exists. Please use a different label", label);
        return
    }

    accounts.push(account);

    let path = get_password_file_path();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Could not save to file");

    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &accounts).unwrap();

    println!("Wrote {} {}", accounts.len(), if accounts.len() == 1 { "byte" } else { "bytes" });
    println!("Success: The entry for '{}' has been saved successfully", label);
}

pub fn get_user_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

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

pub fn file_exists() -> bool{
    let path = get_password_file_path();
    path.exists()
}

pub fn center_align_text(value: &str) -> String {
    let value_len = value.len();
    if value_len >= 5 {
        return format!("{:<width$}", value, width = 5);
    }

    let padding_total = 5 - value_len;
    let left_padding = padding_total / 2;
    let right_padding = padding_total - left_padding;

    format!("{}{}{}", " ".repeat(left_padding), value, " ".repeat(right_padding))
}