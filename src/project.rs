use crate::task::Task;
use crate::time_commitment::TimeCommitment;
use crate::random::random_different_index;
use std::time::Duration;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub note: String,
    pub time_commitment: TimeCommitment,
    tasks: Vec<Task>,
    idx: usize,
}

impl Project {
    pub fn iter(&self) -> std::slice::Iter<'_, Task> {
        self.tasks.iter()
    }

    pub fn start_current_task(&mut self) -> Result<(), &str> {
        self.current_task_mut()?.start();
        Ok(())
    }

    pub fn stop_current_task(&mut self) -> Result<(), &str> {
        self.current_task_mut()?.stop();
        Ok(())
    }

    pub fn current_task(&self) -> Result<&Task, &str> {
        self.tasks.get(self.idx).ok_or("current task idx is invalid")
    }

    pub fn current_task_mut(&mut self) -> Result<&mut Task, &str> {
        self.tasks.get_mut(self.idx).ok_or("current task idx is invalid")
    }

    pub fn set_current_task(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn push_default_task(&mut self) {
        self.tasks.push(Task::default());
    }

    pub fn remove_task(&mut self, idx_to_remove: usize) {
        self.tasks.remove(idx_to_remove);

        // The indexes for all values above idx_to_remove get shifted down one,
        // so if self.idx > idx_to_remove, we need to account for that.
        if self.idx > idx_to_remove {
            self.idx -= 1;
        }
    }

    pub fn has_time_commitment(&self) -> bool {
        self.time_commitment.for_today().as_secs() > 0
    }

    pub fn total_time(&self) -> Duration {
        self.tasks.iter().map(|t| t.elapsed_time).sum::<Duration>()
    }

    pub fn choose_random_task(&mut self) {
        self.idx = random_different_index(&mut self.tasks, self.idx);
    }

    pub fn set_name(&mut self, idx: usize, name: String) -> Result<(), &str> {
        self.tasks.get_mut(idx).ok_or("invalid index")?.name = name;
        Ok(())
    }

    pub fn set_desc(&mut self, idx: usize, desc: String) -> Result<(), &str> {
        self.tasks.get_mut(idx).ok_or("invalid index")?.description = desc;
        Ok(())
    }

    pub fn set_note(&mut self, idx: usize, note: String) -> Result<(), &str> {
        self.tasks.get_mut(idx).ok_or("invalid index")?.note = note;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let mut project = Project {
            name: "project name".to_string(),
            description: "project description".to_string(),
            ..Default::default()
        };

        // there should be no current task yet.
        assert!(project.current_task().is_err());

        project.push_default_task();
        let task1 = project.tasks.last_mut().unwrap();
        task1.name = "task1 name".to_string();
        task1.description = "task1 description".to_string();

        // it should default to the first task.
        assert_eq!(project.current_task().unwrap().name, "task1 name");

        project.push_default_task();
        let task2 = project.tasks.last_mut().unwrap();
        task2.name = "task2 name".to_string();
        task2.description = "task2 description".to_string();

        assert_eq!(project.current_task().unwrap().name, "task1 name");

        // there's 2 tasks + we never return the current task, so we get task2.
        project.choose_random_task();
        assert_eq!(project.current_task().unwrap().name, "task2 name");

        // there's 2 tasks + we never return the current task, so we get task1.
        project.choose_random_task();
        assert_eq!(project.current_task().unwrap().name, "task1 name");
    }
}