#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl Status {
    pub fn next(&self) -> Status {
        match self {
            Status::Todo => Status::Doing,
            Status::Doing => Status::Done,
            Status::Done => Status::Todo,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Status::Todo => "[ ]",
            Status::Doing => "[-]",
            Status::Done => "[x]",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        Task {
            title: title.into(),
            status: Status::Todo,
        }
    }

    pub fn to_line(&self) -> String {
        format!("{} {}", self.status.label(), self.title)
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.len() < 4 {
            return None;
        }
        let (marker, rest) = line.split_at(3);
        let title = rest.trim().to_string();
        if title.is_empty() {
            return None;
        }
        let status = match marker {
            "[ ]" => Status::Todo,
            "[-]" => Status::Doing,
            "[x]" => Status::Done,
            _ => return None,
        };
        Some(Task { title, status })
    }
}
