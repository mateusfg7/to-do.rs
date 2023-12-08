use sqlite3::Connection;

struct TodoModel {
    db: Connection,
}

impl TodoModel {
    fn new() -> Self {
        let db = sqlite3::open("./to-do.db")
            .unwrap_or_else(|_e| panic!("Problem opening the database file."));

        db.execute("    
            CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, completed BOOLEAN NOT NULL CHECK (completed IN (0, 1)));
            ").unwrap();

        Self { db }
    }

    fn add_todo(&self, title: &str) {
        self.db
            .execute(format!(
                "INSERT INTO todos (title, completed) VALUES ('{}', 0)",
                title
            ))
            .unwrap();
    }

    fn list_todos(&self) {
        let mut cursor = self.db.prepare("SELECT * FROM todos").unwrap().cursor();

        println!("");
        while let Some(row) = cursor.next().unwrap() {
            let id = row[0].as_integer().unwrap();
            let title = row[1].as_string().unwrap();
            let completed = if row[2].as_integer().unwrap() == 0 {
                false
            } else {
                true
            };

            println!("[{}] {}. {}", if completed { "X" } else { " " }, id, title);
        }
        println!("");
    }

    fn mark_as_completed(&self, id: i64) {
        self.db
            .execute(format!("UPDATE todos SET completed = 1 WHERE id = {};", id))
            .unwrap();
    }
}

fn main() {
    let todo_model = TodoModel::new();

    todo_model.add_todo("Complete CLI App");
    todo_model.add_todo("Study Rust+SQLite");
    todo_model.add_todo("Review Noisekun PRs");

    todo_model.mark_as_completed(2);

    todo_model.list_todos();
}
