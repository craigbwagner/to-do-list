mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{CLI, Commands};
use task::Task;

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::Add { name, description } => {
            let new_task = Task::new(&name[..], &description);
            match storage::store_task(new_task) {
                Ok(_) => println!("Task '{name}' added succesfully."),
                Err(e) => println!("{e}"),
            }
            ()
        }
        Commands::List => {
            let existing_tasks = storage::read_tasks();
            for task in existing_tasks.iter() {
                println!("{:#?}", task)
            }
        }
        Commands::Complete { id } => {
            Task::complete(id);
        }
        Commands::Remove { id } => {}
    }
}
