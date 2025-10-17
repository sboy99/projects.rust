use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Task {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
