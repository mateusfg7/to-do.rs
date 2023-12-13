mod prelude;

mod database;
use database::Database;

fn main() {
    let db = Database::new();

    db.add("Complete CLI App");
    db.add("Study Rust+SQLite");
    db.add("Review Noisekun PRs");
    db.add("Renamed [FALSE]");

    db.rename(4, "Renamed [TRUE]");
    db.complete(2);
    db.delete(1);
    db.delete(2);

    db.get_all().iter().for_each(|todo| {
        println!(
            "[{}] ({}) {}. {}",
            if todo.is_completed { "X" } else { " " },
            todo.id,
            todo.index,
            todo.title,
        );
    });
}
