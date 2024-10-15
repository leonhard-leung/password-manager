use clap::{Parser, Subcommand};
use rpassword::read_password;
use std::io::{self, Write};

#[derive(Parser)]
#[clap(name = "password-manager")]
#[clap(author = "Leonhard Leung", version, about = "A simple CLI password manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new account
    Add {},

    /// Edit the attributes of an existing account
    Edit {
        #[arg(short = "l", long = "label")]
        label: String,

        #[arg(short = "u", long = "username")]
        username: String,

        #[arg(short = "e", long = "email")]
        email: String,

        #[arg(short = "p", long = "password")]
        password: String,

        #[arg(short = "d", long = "description")]
        description: String,
    },

    /// Generate a new password for an existing account
    GeneratePassword {
        #[arg(short = "len", long = "length")]
        length: u32,

        label: String,
    },

    /// Remove an existing account
    Remove {
        label: String,
    },

    /// List all stored accounts
    List {
        #[arg(short = "sim", long = "simplify")]
        simplify: bool,
    }
}

fn add_account() {
    let label = input("Label: ");
    let username = input("Username: ");
    let email = input("Email: ");

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

    let description = input("Description: ");
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {}) => {
            add_account();
        }
        Some(Commands::Edit { label, username, email, password, description }) => {

        }
        Some(Commands::GeneratePassword { length, label }) => {

        }
        Some(Commands::Remove { label }) => {

        }
        Some(Commands::List { simplify }) => {

        }
        None => {
            println!("No command was provided. Use --help for more information.");
        }
    }
}
