use crate::task::{NewTask, Status, Task};
use rusqlite::{Connection, Result, params};
use std::fs;

pub struct DataBase {
    conn: Connection,
}
impl DataBase {
    pub fn open(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        let sql = "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Uncompleted',
            remember TEXT
        )";

        conn.execute(sql, [])?;
        Ok(Self { conn })
    }
    pub fn new_task(&self, new_task: NewTask) -> Result<Task> {
        self.conn.execute(
            "INSERT INTO tasks (title, status, remember)
VALUES (?1, ?2, ?3)",
            params![new_task.title, new_task.status.as_str(), new_task.remember,],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Task {
            id,
            title: new_task.title,
            status: new_task.status,
            remember: new_task.remember,
        })
    }
    pub fn delete_task(&self, title: &str) -> Result<bool> {
        let deleted = self
            .conn
            .execute("DELETE FROM tasks WHERE title = ?1", [title])?;

        Ok(deleted > 0)
    }
    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, status, remember FROM tasks ORDER BY id")?;

        let tasks = stmt
            .query_map([], |row| {
                let status_text: String = row.get(2)?;

                let status = match status_text.as_str() {
                    "Uncompleted" => Status::Uncompleted,
                    "Starting" => Status::Starting,
                    "Completed" => Status::Completed,
                    _ => return Err(rusqlite::Error::InvalidQuery),
                };

                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    status,
                    remember: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<Task>>>()?;

        Ok(tasks)
    }
    pub fn update_task(&self, title: &str, status: Status) -> Result<bool> {
        let updated = self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE title = ?2",
            rusqlite::params![status.as_str(), title],
        )?;

        Ok(updated > 0)
    }
}
