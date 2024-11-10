use crate::project::Project;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ProjectList {
    projects: Vec<Project>,
    idx: usize,
}

impl ProjectList {
    pub fn iter(&self) -> std::slice::Iter<'_, Project> {
        self.projects.iter()
    }

    pub fn current(&self) -> Result<&Project, &str> {
        self.projects.get(self.idx).ok_or("current project idx is invalid")
    }

    pub fn current_mut(&mut self) -> Result<&mut Project, &str> {
        self.projects.get_mut(self.idx).ok_or("current project idx is invalid")
    }

    pub fn push_default(&mut self) {
        self.projects.push(Project::default());
    }

    pub fn remove(&mut self, idx_to_remove: usize) {
        self.projects.remove(idx_to_remove);

        // The indexes for all values above idx_to_remove get shifted down one,
        // so if self.idx > idx_to_remove, we need to account for that.
        if self.idx > idx_to_remove {
            self.idx -= 1;
        }
    }

    pub fn set_name(&mut self, idx: usize, name: String) -> Result<(), &str> {
        self.projects.get_mut(idx).ok_or("invalid index")?.name = name;
        Ok(())
    }

    pub fn set_desc(&mut self, idx: usize, desc: String) -> Result<(), &str> {
        self.projects.get_mut(idx).ok_or("invalid index")?.description = desc;
        Ok(())
    }

    pub fn set_note(&mut self, idx: usize, note: String) -> Result<(), &str> {
        self.projects.get_mut(idx).ok_or("invalid index")?.note = note;
        Ok(())
    }
}
