use core::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Completed,
    Uncompleted,
    Starting,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Status::Completed => "Completed",
            Status::Uncompleted => "Uncompleted",
            Status::Starting => "Starting",
        };
        write!(f, "{text}")
    }
}
impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Completed => "Completed",
            Status::Uncompleted => "Uncompleted",
            Status::Starting => "Starting",
        }
    }
    pub fn from_str(status_str: &str) -> Status {
        match status_str.to_lowercase().as_str() {
            "completed" => Status::Completed,
            "umcompleted" => Status::Uncompleted,
            "starting" => Status::Starting,
            _ => Status::Uncompleted,
        }
    }
}
#[allow(dead_code)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub status: Status,
    pub remember: Option<String>,
}
pub struct NewTask {
    pub title: String,
    pub status: Status,
    pub remember: Option<String>,
}
impl NewTask {
    pub fn new(t: String, r: Option<String>) -> Self {
        NewTask {
            title: t,
            status: Status::Uncompleted,
            remember: r,
        }
    }
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let remember = self.remember.as_deref().unwrap_or("");
        write!(
            f,
            "title: {}\nstatus: {}\nremember: {}",
            self.title,
            self.status.as_str(),
            remember
        )
    }
}
