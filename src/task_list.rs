use crate::task::Task;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub current_task: usize,
}


