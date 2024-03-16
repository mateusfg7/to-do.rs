mod prelude;

mod database;
use database::Database;

use clap::{Parser, Subcommand};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new to-do")]
    Add {
        #[arg(index = 1, help = "The title of the to-do")]
        title: String,
    },
    #[command(about = "List to-dos")]
    List {
        #[arg(short, long, help = "List all to-dos")]
        all: bool,

        #[arg(short, long, help = "List only completed to-dos")]
        completed: bool,
    },
    #[command(about = "Mark a to-do as done")]
    Done {
        #[arg(index = 1, help = "The id of the to-do")]
        id: usize,
    },
    #[command(about = "Delete a to-do")]
    Delete {
        #[arg(index = 1, help = "The id of the to-do")]
        id: usize,
    },
    #[command(about = "Rename a to-do")]
    Rename {
        #[arg(index = 1, help = "The id of the to-do")]
        id: usize,
        #[arg(index = 2, help = "The new title of the to-do")]
        new_title: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let db = Database::new();

    match &cli.command {
        Some(Commands::Add { title }) => {
            db.add(title);
        }
        Some(Commands::List { all, completed }) => {
            if *all {
                db.get_all().iter().for_each(|todo| {
                    println!(
                        "[{}] ({}) {}. {}",
                        if todo.is_completed { "X" } else { " " },
                        todo.id,
                        todo.index,
                        todo.title,
                    );
                });
            } else if *completed {
                db.get_all().iter().for_each(|todo| {
                    if !todo.is_completed {
                        return;
                    };

                    println!(
                        "[{}] ({}) {}. {}",
                        if todo.is_completed { "X" } else { " " },
                        todo.id,
                        todo.index,
                        todo.title,
                    );
                });
            } else {
                db.get_all().iter().for_each(|todo| {
                    if todo.is_completed {
                        return;
                    }

                    println!(
                        "[{}] ({}) {}. {}",
                        if todo.is_completed { "X" } else { " " },
                        todo.id,
                        todo.index,
                        todo.title,
                    );
                });
            }
        }
        Some(Commands::Done { id }) => {
            db.done(*id);
        }
        Some(Commands::Delete { id }) => {
            db.delete(*id);
        }
        Some(Commands::Rename { id, new_title }) => {
            db.rename(*id, new_title);
        }

        None => {}
    }
}
