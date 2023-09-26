use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    title: String,
    description: String,
    completed: bool,
}

impl Project {
    pub fn new() -> Self {
        Project { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn mark_task_complete(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        }
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        Task {
            title,
            description,
            completed: false,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
