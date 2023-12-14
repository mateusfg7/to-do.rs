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
    Add {
        #[arg(index = 1)]
        title: String,
    },
    List {
        #[arg(short, long)]
        all: bool,

        #[arg(short, long)]
        completed: bool,
    },
    Done {
        #[arg(index = 1)]
        id: usize,
    },
    Delete {
        #[arg(index = 1)]
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let db = Database::new();

    // db.add("Complete CLI App");
    // db.add("Study Rust+SQLite");
    // db.add("Review Noisekun PRs");
    // db.add("Renamed [FALSE]");

    // db.rename(4, "Renamed [TRUE]");
    // db.complete(2);
    // db.delete(1);
    // db.delete(2);

    match &cli.command {
        Some(Commands::Add { title }) => {
            db.add(title);
            db.add(title);
            db.add(title);
            db.done(2);
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
        None => {}
    }
}
