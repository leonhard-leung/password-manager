use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "password-manager")]
#[clap(author = "Leonhard Leung", version, about = "A simple CLI password manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new account and its password
    Add {
        account: String,
        password: String,
    },
    /// Get the password for an account
    Get {
        account: String,
    },
    /// Display the list of saved accounts
    List {},
    /// Generate a random password for an account
    Gen {
        length: usize,
    }
}

fn main() {
    let cli = Cli::parse();
}
