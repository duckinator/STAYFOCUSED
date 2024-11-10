//use iced::widget::{Column, TextInput};
//use iced::widget::{button, column, text, text_input};
use iced::widget::text;
use iced::Element;
use serde::{Deserialize, Serialize};

mod project;
mod project_list;
mod random;
mod task;
mod time_commitment;

pub use project::Project;
pub use project_list::ProjectList;
pub use task::Task;
pub use time_commitment::TimeCommitment;

#[derive(Debug)]
enum Message {
    ProjectAdd,
    ProjectRemove(usize),
    ProjectSetName(usize, String),
    ProjectSetDesc(usize, String),
    ProjectSetNote(usize, String),
    TaskAdd,
    TaskRemove(usize),
    TaskSetName(usize, String),
    TaskSetDesc(usize, String),
    TaskSetNote(usize, String),
    SwitchView(View),
}

#[derive(Debug, Default, Deserialize, Serialize)]
enum View {
    #[default]
    Task,
    Project,
    ProjectList,
}

#[derive(Default)]
struct App {
    projects: ProjectList,
    view: View,
}

impl App {
    fn set_view(&mut self, view: View) {
        self.view = view;
    }
}

fn update_inner(app: &mut App, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    match message {
        Message::ProjectAdd => app.projects.push_default(),
        Message::ProjectRemove(idx) => app.projects.remove(idx),
        Message::ProjectSetName(idx, name) => app.projects.set_name(idx, name)?,
        Message::ProjectSetDesc(idx, desc) => app.projects.set_desc(idx, desc)?,
        Message::ProjectSetNote(idx, note) => app.projects.set_note(idx, note)?,
        Message::TaskAdd => app.projects.current_mut()?.push_default_task(),
        Message::TaskRemove(idx) => app.projects.current_mut()?.remove_task(idx),
        Message::TaskSetName(idx, name) => app.projects.current_mut()?.set_name(idx, name)?,
        Message::TaskSetDesc(idx, desc) => app.projects.current_mut()?.set_desc(idx, desc)?,
        Message::TaskSetNote(idx, note) => app.projects.current_mut()?.set_note(idx, note)?,
        Message::SwitchView(view) => app.set_view(view),
    }

    Ok(())
}

fn update(app: &mut App, message: Message) {
    match update_inner(app, message) {
        Ok(()) => {},
        Err(msg) => eprintln!("{}", msg),
    }
}

fn view(_app: &App) -> Element<Message> {
    text("hello").into()
}

fn main() -> iced::Result {
    iced::run("STAYFOCUSED", update, view)
}
