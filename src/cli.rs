use crate::task::Status;

pub const HELP_MESSAGE: &'static str = "\
Todo — Task Manager

Usage:
  todo <cmd>

Cmd:
  add     Add a task
  rm      Remove a task by title
  up      Update a task by title
  show    Show all tasks
  h       Show this message

Avaible Status:
  Uncompleted
  Starting
  Completed

Exemples:
  todo add
  todo rm
  todo up
  todo show";

pub fn get_input_add() -> (String, String) {
    let mut input = String::new();
    let mut input2 = String::new();
    println!("Enter a title for task");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Enter a valid input");
    let title_trim = input.trim();
    println!("Enter a remeber for task");
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Enter a valid input");
    let remeber_trim = input2.trim();
    (title_trim.to_string(), remeber_trim.to_string())
}
pub fn get_input_rm() -> String {
    let mut input = String::new();
    println!("Enter the title of a task to remove: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Enter a valid input");
    let title_trim = input.trim().to_string();
    title_trim
}
pub fn get_input_up() -> (String, Status) {
    let mut input = String::new();
    let mut input_status = String::new();
    println!("Enter the title of a task to update: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Enter a valid input");
    let title_trim = input.trim().to_string();
    println!("Enter the currently status of a Task: (Completed, Starting, Umcompleted)");
    std::io::stdin()
        .read_line(&mut input_status)
        .expect("Enter a valid input");
    let status_str = input_status.trim();
    let status = Status::from_str(status_str);
    (title_trim, status)
}
