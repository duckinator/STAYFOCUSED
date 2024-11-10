use serde::{Deserialize, Serialize};

mod project;
mod random;
mod task;
mod time_commitment;

use project::Project;

#[derive(Default, Debug, Deserialize, Serialize)]
struct State {
    pub projects: Vec<Project>,
    current_project_idx: usize,
}

impl State {
    pub fn current_project(&self) -> Option<&Project> {
        self.projects.get(self.current_project_idx)
    }

    pub fn current_project_mut(&mut self) -> Option<&mut Project> {
        self.projects.get_mut(self.current_project_idx)
    }

    pub fn add_default_project(&mut self) {
        self.projects.push(Project::default());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
