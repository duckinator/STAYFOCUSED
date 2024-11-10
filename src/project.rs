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
    current_task_idx: usize,
}

impl Project {
    pub fn start_current_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_task_mut().ok_or("Project has no tasks")?.start();
        Ok(())
    }

    pub fn stop_current_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.current_task_mut().ok_or("Project has no tasks")?.stop();
        Ok(())
    }

    pub fn current_task(&self) -> Option<&Task> {
        self.tasks.get(self.current_task_idx)
    }

    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.tasks.get_mut(self.current_task_idx)
    }

    pub fn set_current_task(&mut self, idx: usize) {
        self.current_task_idx = idx;
    }

    pub fn add_default_task(&mut self) {
        self.tasks.push(Task::default());
    }

    pub fn has_time_commitment(&self) -> bool {
        self.time_commitment.for_today().as_secs() > 0
    }

    pub fn total_time(&self) -> Duration {
        self.tasks.iter().map(|t| t.elapsed_time).sum::<Duration>()
    }

    pub fn choose_random_task(&mut self) {
        self.current_task_idx = random_different_index(&mut self.tasks, self.current_task_idx);
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
        assert_eq!(project.current_task(), None);

        project.add_default_task();
        let task1 = project.tasks.last_mut().unwrap();
        task1.name = "task1 name".to_string();
        task1.description = "task1 description".to_string();

        // it should default to the first task.
        assert_eq!(project.current_task().unwrap().name, "task1 name");

        project.add_default_task();
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
