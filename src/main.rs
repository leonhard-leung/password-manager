use clap::{Parser, Subcommand};

mod manager;
mod util;

#[derive(Parser)]
#[clap(name = "Password Manager")]
#[clap(
    author = "Leonhard Leung",
    version = "0.1.0",
    about = "A simple CLI password manager"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new account
    Add {},

    /// Edit the attributes of an existing account
    Edit { label: String, },

    /// Generate a new password for an existing account
    GenPass {
        #[arg(short = 'l', long = "length")]
        length: u32,

        label: String,
    },

    /// Remove an existing account
    Remove { label: String },

    /// List all stored accounts
    List {
        #[arg(short = 's', long = "simplify")]
        simplify: bool,
    },
}

#[allow(unused_variables)]
fn main() {
    if !util::file_exists() {
        println!("Created necessary files and directories");
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {}) => {
            manager::add();
        }

        Some(Commands::Edit { label}) => {
            manager::edit(label);
        }

        Some(Commands::GenPass { length, label }) => {}

        Some(Commands::Remove { label }) => {
            manager::remove(label);
        }

        Some(Commands::List { simplify }) => {
            manager::display_accounts(simplify);
        }

        None => {
            println!("No command was provided. Use --help for more information.");
        }
    }
}
