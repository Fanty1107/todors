use crate::db::DataBase;
use crate::task::NewTask;
use rusqlite::Result;
use std::env;
use std::path::PathBuf;
mod cli;
mod db;
mod task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let home = std::env::var_os("HOME").expect("Não foi possível encontrar o diretório HOME");

    let db_path = PathBuf::from(home)
        .join(".local")
        .join("todo")
        .join("todo.db");
    let db = DataBase::open(&db_path)?;
    match args.get(1).map(|farg| farg.as_str()) {
        Some("add") => {
            let info_task = cli::get_input_add();
            let new_task = NewTask::new(info_task.0, Some(info_task.1));
            db.new_task(new_task)?;
            Ok(())
        }
        Some("rm") => {
            let task_title = cli::get_input_rm();
            if db.delete_task(&task_title).unwrap() {
                println!("{task_title} deleted");
            } else {
                println!("No task with this title");
            }
            Ok(())
        }
        Some("up") => {
            let task_info = cli::get_input_up();
            if db.update_task(&task_info.0, task_info.1).unwrap() {
                println!("{} updated", task_info.0);
            } else {
                println!("No task with this title");
            }
            Ok(())
        }
        Some("show") => {
            for task in db.list_tasks().unwrap() {
                println!("{task}");
            }
            Ok(())
        }
        Some("h") => {
            println!("{}", cli::HELP_MESSAGE);
            Ok(())
        }
        None => {
            eprintln!("no args passed");
            Ok(())
        }
        Some(_) => {
            eprintln!("See h for help usage");
            Ok(())
        }
    }
}
