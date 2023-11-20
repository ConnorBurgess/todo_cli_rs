use clap::{Parser, Subcommand};
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{self, Value};
use serde::{Deserialize, Serialize};
use serial_test::serial;
mod tasks;
mod error;

use error::TaskError;
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
    List,
}

fn main() -> Result<(), TaskError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => {
            println!("Added a new task: {}", description);
            add_task(description)?;
        },
        Commands::List => {
            list_tasks()?;
        },

    }

    Ok(())
}

fn add_task(description: String) -> Result<(), TaskError> {
    let mut tasks = load_tasks("tasks.json")?;
    tasks.push(Task::new(description));
    save_tasks("tasks.json", &tasks)
}
fn list_tasks() -> Result<String, TaskError> {
    let tasks = load_tasks("tasks.json")?;
    let mut output = String::new();

    for (index, task) in tasks.iter().enumerate() {
        output.push_str(&format!("{}: {}\n", index + 1, task.description));
    }

    Ok(output)
}


fn load_tasks(filename: &str) -> Result<Vec<Task>, TaskError> {
    let mut file = File::open(filename).map_err(TaskError::Io)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(TaskError::Io)?;
    let tasks = serde_json::from_str(&contents).map_err(TaskError::Json)?;

    Ok(tasks)
}


fn save_tasks(filename: &str, tasks: &[Task]) -> Result<(), TaskError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;
    let json = serde_json::to_string(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn test_add_task() {
        let description = "Test task".to_string();
        println!("Error occurred: {}", description);

        let result = add_task(description.clone());

        if let Err(e) = &result {
            println!("Error occurred: {}", e);
        }

        assert!(result.is_ok());

        // Verify the task was added
        let tasks = load_tasks("tasks.json").unwrap();
        assert_eq!(tasks.last().unwrap().description, description);

        // Cleanup: remove the test task from the file
        let tasks = tasks.into_iter().filter(|task| task.description != description).collect::<Vec<_>>();
        save_tasks("tasks.json", &tasks).unwrap();
    }

    #[test]
    #[serial]
    fn test_list_tasks() {
        let description = "Test task".to_string();
        add_task(description.clone()).unwrap();

        let output = list_tasks().unwrap();
        assert!(output.contains(&description));

        // Cleanup: remove the test task from the file
        let tasks = load_tasks("tasks.json").unwrap();
        let tasks = tasks.into_iter().filter(|task| task.description != description).collect::<Vec<_>>();
        save_tasks("tasks.json", &tasks).unwrap();
    }

}