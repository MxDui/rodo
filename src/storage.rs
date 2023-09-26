use std::fs;
use crate::project::Project;
use serde_json;

pub fn save_project(project: &Project, filename: &str) -> bool {
    let serialized = serde_json::to_string(project).unwrap();

    fs::write(filename, serialized).is_ok()
}

pub fn load_project(filename: &str) -> Result<Project, &'static str> {
    match fs::read_to_string(filename) {
        Ok(data) => {
            let project: Project = serde_json::from_str(&data).unwrap();
            Ok(project)
        }
        Err(_) => Err("Failed to load the project.")
    }
}
