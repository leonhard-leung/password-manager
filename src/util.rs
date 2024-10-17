use std::path::Path;
use std::fs::{File, OpenOptions};
use std::fs::create_dir_all;
use std::io::{BufReader, BufWriter, Read};
use serde_json::to_writer_pretty;
use crate::manager::Account;

pub fn file_exists() -> bool{
    let path = Path::new("C:\\Password Manager\\data\\passwords.json");

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("Could not create directories");
        }
    }

    if !path.exists() {
        File::create(path).expect("Could not create file");
    }

    path.exists()
}

pub fn get_data() -> serde_json::Result<Vec<Account>> {
    let file = File::open("C:\\Password Manager\\data\\passwords.json").expect("Could not open file");
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Could not read file");

    if contents.is_empty() {
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

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("C:\\Password Manager\\data\\passwords.json")
        .expect("Could not create file");

    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &accounts).unwrap();

    println!("Wrote {} bytes", accounts.len());
    println!("Success: The entry for '{}' has been saved successfully", label);
}