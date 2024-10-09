use eframe::egui;
use egui_extras::{TableBuilder, Column};

mod task; // used by task_list
mod task_list;
use task_list::TaskList;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("STAYFOCUSED", native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))))
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
enum View {
    #[default]
    ActiveTask,
    TaskList,
}

#[derive(Default)]
struct MainApp {
    view: View,
    tasks: TaskList,
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
            if let Some(tasks) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.tasks = tasks;
            }
        }

        slf
    }

    fn add_sized_btn(ui: &mut egui::Ui, name: &str) -> egui::Response {
        let button_size = [100.0, 30.0];
        ui.add_sized(button_size, egui::Button::new(name))
    }

    // [ linear clock                             ]
    // [ current task                             ]
    // | notes                                    |
    // [Pause][wrap up indicator] [ Next ] [ List ]
    fn update_active_task(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                            if self.tasks.current().is_tracking_time() {
                                if Self::add_sized_btn(ui, "Pause").clicked() {
                                    self.tasks.current_mut().stop();
                                }
                            } else {
                                if Self::add_sized_btn(ui, "Start").clicked() {
                                    self.tasks.current_mut().start();
                                }
                            }
                        });

                        row.col(|ui| {
                            self.tasks.current_mut().tick();
                            let time_str = self.tasks.current().elapsed_time_str();
                            ui.label(time_str);
                        });

                        row.col(|ui| {
                            if Self::add_sized_btn(ui, "Next").clicked() && !self.tasks.is_empty() {
                                self.tasks.choose_random();
                            }
                        });

                        row.col(|ui| {
                            if Self::add_sized_btn(ui, "Tasks").clicked() {
                                self.view = View::TaskList;
                            }
                        });
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let text = &self.tasks.current().description;

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

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.tasks.current_mut().note);
                });
            });
        });

        // If task is running, request a redraw to update the timer.
        if self.tasks.current().is_tracking_time() {
            ctx.request_repaint_after_secs(0.2);
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
                            if Self::add_sized_btn(ui, "Add").clicked() {
                                self.tasks.push_default();
                            }
                        });

                        row.col(|ui| {
                            if Self::add_sized_btn(ui, "Done").clicked() {
                                self.view = View::ActiveTask;
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
                    for (i, task) in self.tasks.iter_mut().enumerate() {
                        body.row(interact_size.y, |mut row| {
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut task.description);
                            });

                            row.col(|ui| {
                                ui.label(task.elapsed_time_str());
                            });

                            row.col(|ui| {
                                if Self::add_sized_btn(ui, "Delete").clicked() {
                                    deferred_removal = Some(i);
                                }
                            });
                        });
                    }
                    if let Some(idx) = deferred_removal {
                        self.tasks.remove(idx)
                    }
                });
        });
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.tasks.is_empty() {
            self.view = View::TaskList;
        }

        match &self.view {
            View::ActiveTask   => self.update_active_task(ctx, frame),
            View::TaskList     => self.update_task_list(ctx, frame),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.tasks);
    }
}
