use eframe::egui;
use egui_extras::{TableBuilder, Column};
use rand::seq::SliceRandom;
use rand;
use std::time::Duration;
mod task;
use task::Task;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("STAYFOCUSED", native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))))
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

        use egui::FontFamily::Proportional;
        use egui::FontId;
        use egui::TextStyle::*;

        let mut style = (*cc.egui_ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(20.0, Proportional)),
            (Name("Heading2".into()), FontId::new(20.0, Proportional)),
            (Name("Context".into()), FontId::new(20.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ].into();

        //style.spacing.item_spacing = egui::vec2(20.0, 20.0);
        //style.spacing.button_padding = egui::vec2(10.0, 10.0);
        style.spacing.interact_size = egui::vec2(30.0, 30.0);

        cc.egui_ctx.set_style(style);


        let mut slf = Self::default();

        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }

    fn add_sized_btn(&mut self, ui: &mut egui::Ui, name: &str) -> egui::Response {
        let button_size = [100.0, 30.0];
        ui.add_sized(button_size, egui::Button::new(name))
    }

    // [ linear clock                             ]
    // [ current task                             ]
    // | description                              |
    // [Pause][wrap up indicator] [ Next ] [ List ]
    fn update_active_task(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current = self.state.current_task;
        let interact_size = ctx.style().spacing.interact_size;

        egui::TopBottomPanel::bottom("active_task_bottom").show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::exact(100.0))
                .column(Column::remainder())
                .column(Column::exact(100.0))
                .column(Column::exact(100.0))
                .body(|mut body| {
                    body.row(interact_size.y, |mut row| {
                        row.col(|ui| {
                            if self.state.tasks[current].start_instant.is_some() {
                                if self.add_sized_btn(ui, "Pause").clicked() {
                                    self.state.tasks[current].stop();
                                }
                            } else {
                                if self.add_sized_btn(ui, "Start").clicked() {
                                    self.state.tasks[current].start();
                                }
                            }
                        });

                        row.col(|ui| {
                            let time_str = self.state.tasks[current].elapsed_time_str();
                            ui.label(time_str);
                        });

                        row.col(|ui| {
                            if self.add_sized_btn(ui, "Next").clicked() && self.state.tasks.len() > 1 {
                                let mut indexes: Vec<_> = (0..self.state.tasks.len()).collect();
                                let pos = indexes.iter().position(|v| *v == self.state.current_task).unwrap();
                                indexes.remove(pos);
                                self.state.current_task = *indexes.choose(&mut rand::thread_rng()).unwrap();
                            }
                        });

                        row.col(|ui| {
                            if self.add_sized_btn(ui, "Tasks").clicked() {
                                self.state.view = View::TaskList;
                            }
                        });
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let text = &self.state.tasks[current].description;

            let style = ctx.style();
            let mut layout_job = egui::text::LayoutJob::default();
            egui::RichText::new(text)
                .color(style.visuals.text_color())
                .size(40.0)
                .append_to(
                    &mut layout_job,
                    &style,
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );

            ui.label(layout_job);
        });

        // If task is running, request a redraw every half-second
        // to update the timer.
        if self.state.tasks[current].start_instant.is_some() {
            ctx.request_repaint_after(Duration::new(0, 500));
        }
    }

    // [ linear clock                                  ]
    // Tasks:
    // [ pri ] [ duration ] [ task ] [ Edit ] [ Delete ]
    // [                      Add                      ]
    fn update_task_list(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let interact_size = ctx.style().spacing.interact_size;

        egui::TopBottomPanel::bottom("task_list_bottom").show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::remainder())
                .column(Column::exact(100.0))
                .column(Column::exact(100.0))
                //.header(30.0, |mut header| {})
                .body(|mut body| {
                    body.row(interact_size.y, |mut row| {
                        row.col(|_ui| { });

                        row.col(|ui| {
                            if self.add_sized_btn(ui, "Add").clicked() {
                                self.state.tasks.push(Task::default());
                            }
                        });

                        row.col(|ui| {
                            if self.add_sized_btn(ui, "Done").clicked() {
                                self.state.view = View::ActiveTask;
                            }
                        });
                    });

                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::remainder())
                .column(Column::exact(100.0))
                .column(Column::exact(100.0))
                .header(interact_size.x, |mut header| {
                    header.col(|ui| { ui.heading("Task"); });
                    header.col(|ui| { ui.heading("Elapsed"); });
                    header.col(|ui| { ui.heading("Actions"); });
                })
                .body(|mut body| {
                    let mut deferred_removal: Option<usize> = None;
                    for i in 0..self.state.tasks.len() {
                        body.row(interact_size.y, |mut row| {
                            let task = &mut self.state.tasks[i];
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut task.description);
                            });

                            row.col(|ui| {
                                ui.label(task.elapsed_time_str());
                            });

                            row.col(|ui| {
                                if self.add_sized_btn(ui, "Delete").clicked() {
                                    deferred_removal = Some(i);
                                }
                            });
                        });
                    }
                    if let Some(idx) = deferred_removal {
                        self.state.tasks.remove(idx);
                        if self.state.current_task > idx {
                            self.state.current_task -= 1;
                        }
                    }
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
