use eframe::egui;
use rand::seq::SliceRandom;
use rand;
use std::time::{Duration, Instant};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("STAYFOCUSED", native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))))
}

// [ pri ] [ duration ] [ task ] [ Edit ] [ Delete ]
#[derive(Default, serde::Deserialize, serde::Serialize)]
struct Task {
    //priority: Priority,
    elapsed_time: Duration,
    name: String,
    description: String,
    #[serde(skip)]
    start_instant: Option<Instant>,
}

impl Task {
    fn start(&mut self) {
        if self.start_instant.is_none() {
            self.start_instant = Some(Instant::now());
        }
    }

    fn stop(&mut self) {
        if self.start_instant.is_some() {
            self.tick();
            self.start_instant = None;
        }
    }

    fn tick(&mut self) {
        if self.start_instant.is_none() {
            return;
        }

        let now = Instant::now();
        let difference = now.duration_since(self.start_instant.unwrap());

        // Mostly just so it's not updating every fucking frame.
        if difference.as_secs() < 1 {
            return;
        }

        self.elapsed_time += difference;

        self.start_instant = Some(now);
    }

    fn elapsed_time_str(&self) -> String {
        let time = self.elapsed_time.as_secs();
        let mins = time / 60;
        let secs = time % 60;

        let hours = mins / 60;
        let mins = mins % 60;

        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
enum View {
    ActiveTask,
    TaskList,
}

impl Default for View {
    fn default() -> Self { View::ActiveTask }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct State {
    view: View,
    tasks: Vec<Task>,
    current_task: usize,
}

#[derive(Default)]
struct MainApp {
    state: State,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut slf = Self::default();

        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }

    // [ linear clock                             ]
    // [ current task                             ]
    // | description                              |
    // [Pause][wrap up indicator] [ Next ] [ List ]
    fn update_active_task(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("active-task").show(ui, |ui| {
                let current = self.state.current_task;

                self.state.tasks[current].tick();

                // TODO: Clock?
                ui.heading(&self.state.tasks[current].name);
                ui.end_row();

                ui.label(&self.state.tasks[current].description);
                ui.end_row();

                if self.state.tasks[current].start_instant.is_some() {
                    if ui.button("Pause").clicked() { self.state.tasks[current].stop(); }
                } else {
                    if ui.button("Start").clicked() { self.state.tasks[current].start(); }
                }

                let time_str = self.state.tasks[current].elapsed_time_str();
                ui.label(time_str);

                if ui.button("Next").clicked() && self.state.tasks.len() > 1 {
                    let mut indexes: Vec<_> = (0..self.state.tasks.len()).collect();
                    let pos = indexes.iter().position(|v| *v == self.state.current_task).unwrap();
                    indexes.remove(pos);
                    self.state.current_task = *indexes.choose(&mut rand::thread_rng()).unwrap();
                }

                if ui.button("Tasks").clicked() {
                    self.state.view = View::TaskList;
                }
                ui.end_row();

                // If task is running, request a redraw every half-second
                // to update the timer.
                if self.state.tasks[current].start_instant.is_some() {
                    ctx.request_repaint_after(Duration::new(0, 500));
                }
            });
        });
    }

    // [ linear clock                                  ]
    // Tasks:
    // [ pri ] [ duration ] [ task ] [ Edit ] [ Delete ]
    // [                      Add                      ]
    fn update_task_list(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");
            egui::Grid::new("active-task").show(ui, |ui| {
                ui.label("Name");
                ui.label("Description");
                ui.label("Elapsed");
                ui.label("Actions");
                ui.end_row();

                let mut deferred_removal: Option<usize> = None;
                for i in 0..self.state.tasks.len() {
                    let task = &mut self.state.tasks[i];
                    ui.text_edit_singleline(&mut task.name);
                    ui.text_edit_singleline(&mut task.description);

                    ui.label(task.elapsed_time_str());

                    if ui.button("Delete").clicked() {
                        deferred_removal = Some(i);
                    }
                    ui.end_row();
                }
                if let Some(idx) = deferred_removal {
                    self.state.tasks.remove(idx);
                    if self.state.current_task > idx {
                        self.state.current_task -= 1;
                    }
                }


                ui.end_row();

                // Is there some more appropriate way to say "this cell is empty?"
                ui.label("");
                ui.label("");

                if ui.button("Add").clicked() {
                    self.state.tasks.push(Task::default());
                }
                if ui.button("Done").clicked() {
                    self.state.view = View::ActiveTask;
                }
                ui.end_row();
            });
        });
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.state.tasks.len() == 0 {
            self.state.view = View::TaskList;
        }

        match &self.state.view {
            View::ActiveTask   => self.update_active_task(ctx, frame),
            View::TaskList     => self.update_task_list(ctx, frame),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
}
