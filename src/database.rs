use crate::prelude::*;

use rand::{distributions::Alphanumeric, Rng};
use sqlite3::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let db = sqlite3::open("./todo.db")
            .unwrap_or_else(|_e| panic!("Problem opening the database file."));

        db.execute("    
          CREATE TABLE IF NOT EXISTS todos (id TEXT NOT NULL, title TEXT NOT NULL, is_completed BOOLEAN NOT NULL CHECK (is_completed IN (0, 1)), created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
          ").unwrap_or_else(|e| panic!("{}", e.message.unwrap_or("error while creating tables on database".to_string())));

        Self { conn: db }
    }

    pub fn add(&self, title: &String) {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        self.conn
            .execute(format!(
                "INSERT INTO todos (id, title, is_completed) VALUES ('{}', '{}', 0)",
                id, title
            ))
            .unwrap_or_else(|e| {
                panic!(
                    "{}",
                    e.message
                        .unwrap_or("error while creating a new to-do".to_string())
                )
            });
    }

    pub fn done(&self, index: usize) {
        let target = &self.get_all()[index - 1];

        self.conn
            .execute(format!(
                "UPDATE todos SET is_completed = 1 WHERE id = '{}';",
                target.id
            ))
            .unwrap();
    }

    pub fn rename(&self, index: usize, new_title: &str) {
        let target = &self.get_all()[index - 1];

        self.conn
            .execute(format!(
                "UPDATE todos SET title = '{}' WHERE id = '{}';",
                new_title, target.id
            ))
            .unwrap();
    }

    pub fn delete(&self, index: usize) {
        let target = &self.get_all()[index - 1];

        self.conn
            .execute(format!("DELETE FROM todos WHERE id = '{}';", target.id))
            .unwrap();
    }

    pub fn get_all(&self) -> Vec<Todo> {
        let mut todos: Vec<Todo> = vec![];
        let query = "SELECT ROW_NUMBER() OVER (ORDER BY created_at) row_index,id,title,is_completed,created_at FROM todos";

        self.conn
            .iterate(query, |row| {
                let value = row
                    .iter()
                    .map(|&(_column, value)| value.unwrap())
                    .collect::<Vec<&str>>();

                todos.push(Todo {
                    index: value[0].parse::<i64>().unwrap(),
                    id: value[1].to_string(),
                    title: value[2].to_string(),
                    is_completed: value[3] == "1",
                    created_at: value[4].to_string(),
                });
                true
            })
            .unwrap();

        todos
    }
}
