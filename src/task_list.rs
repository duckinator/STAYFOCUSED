use rand::seq::SliceRandom;
use crate::task::Task;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct TaskList {
    tasks: Vec<Task>,
    current_task_idx: usize,
}

impl TaskList {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Task> {
        self.tasks.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn current(&self) -> &Task {
        &self.tasks[self.current_task_idx]
    }

    pub fn current_mut(&mut self) -> &mut Task {
        &mut self.tasks[self.current_task_idx]
    }

    pub fn set_current(&mut self, idx: usize) {
        self.current_task_idx = idx;
    }

    pub fn push_default(&mut self) {
        self.tasks.push(Task::default());
    }

    pub fn remove(&mut self, idx: usize) {
        self.tasks.remove(idx);
        if self.current_task_idx > idx {
            self.current_task_idx -= 1;
        }
    }

    pub fn choose_random(&mut self) {
        let mut indexes: Vec<_> = (0..self.tasks.len()).collect();
        if let Some(pos) = indexes.iter().position(|v| *v == self.current_task_idx) {
            indexes.remove(pos);
            if let Some(idx) = indexes.choose(&mut rand::thread_rng()) {
                self.set_current(*idx);
            } else {
                eprintln!("TaskList has {} items, choose_random() is useless.", self.tasks.len());
            }
        } else {
            eprintln!("TaskList has {} items, but current_task_idx is {}", self.tasks.len(), self.current_task_idx);
        }
    }
}
