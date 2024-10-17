use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub label: String,
    pub email: String,
    pub username: String,
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

pub fn add(label: String, email: String, username: String, password: String, description: String) {
    let new_account = Account {
        label,
        email,
        username,
        password,
        description,
    };

    append_to_file(new_account).unwrap();
}

pub fn display() {
    let accounts = read_from_file().unwrap();

    for account in &accounts {
        println!("{:#?}", account.display())
    }
}

fn read_from_file() -> Result<Vec<Account>> {
    let file = File::open("data/passwords.json").expect("file not found");
    let reader = BufReader::new(file);

    let accounts = serde_json::from_reader(reader)?;

    Ok(accounts)
}

fn append_to_file(account: Account) -> Result<()> {
    let mut accounts: Vec<Account> = read_from_file()?;
    let label = account.label.clone();

    if accounts.iter().any(|existing_account| existing_account.label == account.label) {
        println!("Error: An entry with the label '{}' already exists. Please use a different label", account.label);
        return Ok(());
    }

    accounts.push(account);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data/passwords.json")
        .expect("unable to open file for writing");

    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &accounts)?;

    println!("Success: The entry for '{}' has been saved successfully.", label);
    Ok(())
}

