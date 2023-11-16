use clap::{Parser, Subcommand};
use std::io;

mod tasks;
use tasks::Task;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a new task
    Add {
        description: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => {
            println!("Added a new task");
            // Call function to add task; add_task(description);
        }
    }

    Ok(())
}