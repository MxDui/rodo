mod project;
mod ui;
mod storage;

use project::Project;

fn main() {
    let mut project = Project::new();

    loop {
        match ui::show_menu() {
            ui::MenuChoice::NewProject => {
                project = Project::new();
                println!("New project created.");
            }
            ui::MenuChoice::AddTask => {
                let task = ui::get_task_details();
                project.add_task(task);
                println!("Task added.");
            }
            ui::MenuChoice::ListTasks => {
                ui::display_tasks(&project);
            }
            ui::MenuChoice::CompleteTask => {
                if let Some(index) = ui::get_task_to_mark_complete(&project) {
                    project.mark_task_complete(index);
                    println!("Task marked as complete.");
                }
            }
            ui::MenuChoice::Save => {
                let filename = ui::get_filename();
                if storage::save_project(&project, &filename) {
                    println!("Project saved successfully.");
                } else {
                    println!("Error saving project.");
                }
            }
            ui::MenuChoice::Load => {
                let filename = ui::get_filename();
                match storage::load_project(&filename) {
                    Ok(loaded_project) => {
                        project = loaded_project;
                        println!("Project loaded successfully.");
                    }
                    Err(err) => {
                        println!("Error loading project: {}", err);
                    }
                }
            }
            ui::MenuChoice::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
