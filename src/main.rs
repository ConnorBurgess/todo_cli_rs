use clap::{Parser, Subcommand};
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{self, Value};
use serde::{Deserialize, Serialize};

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
                add_task(description)?;
            println!("Added a new task");
        }
    }

    Ok(())
}

fn add_task(description: String) -> io::Result<()> {
    let mut tasks = load_tasks("tasks.json")?;
    tasks.push(Task::new(description));
    save_tasks("tasks.json", &tasks)
}

fn load_tasks(filename: &str) -> std::io::Result<Vec<Task>> {
    let file = File::open(filename);

    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(Vec::new())
        },
        Err(e) => Err(e),
    }
}


fn save_tasks(filename: &str, tasks: &[Task]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;
    let json = serde_json::to_string(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}