use std::io;
use crate::project::{Project, Task};

pub enum MenuChoice {
    NewProject,
    AddTask,
    ListTasks,
    CompleteTask,
    Save,
    Load,
    Exit,
}

pub fn show_menu() -> MenuChoice {
    loop {
        println!("Project Manager CLI");
        println!("1. Create a new project");
        println!("2. Add a task");
        println!("3. List tasks");
        println!("4. Mark a task as completed");
        println!("5. Save project to .prm file");
        println!("6. Load project from .prm file");
        println!("7. Exit");

        match input("Enter your choice: ").as_ref() {
            "1" => return MenuChoice::NewProject,
            "2" => return MenuChoice::AddTask,
            "3" => return MenuChoice::ListTasks,
            "4" => return MenuChoice::CompleteTask,
            "5" => return MenuChoice::Save,
            "6" => return MenuChoice::Load,
            "7" => return MenuChoice::Exit,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

pub fn get_task_details() -> Task {
    let title = input("Enter task title: ");
    let description = input("Enter task description: ");
    Task::new(title, description)
}

pub fn display_tasks(project: &Project) {
    if project.tasks().is_empty() {
        println!("No tasks in the project.");
    } else {
        for (index, task) in project.tasks().iter().enumerate() {
            let status = if task.is_completed() { "Completed" } else { "Pending" };
            println!("{}: {} - {} [{}]", index + 1, task.title(), task.description(), status);
        }
    }
}

pub fn get_task_to_mark_complete(project: &Project) -> Option<usize> {
    let index_str = input("Enter task number to mark as completed: ");
    if let Ok(index) = index_str.parse::<usize>() {
        if index > 0 && index <= project.tasks().len() {
            Some(index - 1)
        } else {
            println!("Invalid task number.");
            None
        }
    } else {
        println!("Please enter a valid number.");
        None
    }
}

pub fn get_filename() -> String {
    input("Enter filename (with .prm extension): ")
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
