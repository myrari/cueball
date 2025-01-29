//use std::collections::HashSet;
use egui_extras::{TableBuilder,TableRow,Column};
use egui::RichText;
use crate::data::{Cue, CueList};

#[derive(Debug)]
pub struct CueballApp {
    state: AppState,
}

impl Default for CueballApp {
    fn default() -> Self {
        CueballApp {
            state: AppState {
                project: Project {
                    name: String::from("Untitled.cbp"),
                    cues: CueList::new(),
                    selected_cue: None,
                },
            },
        }
    }
}

impl CueballApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        // do nothing here for now
        Self { state }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub project: Project,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,

    pub cues: CueList,

    selected_cue: Option<u64>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("untitled.cueball"),
            cues: CueList::new(),
            selected_cue: None,
        }
    }
}

impl eframe::App for CueballApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // menu bar
            egui::menu::bar(ui, |ui| {
                // file menu and spacer
                ui.menu_button("File", |ui| {
                    // theme widget
                    egui::widgets::global_theme_preference_buttons(ui);
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("inspector_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // tab ribbon
                    ui.horizontal(|ui| {
                        ui.set_height(16.);

                        ui.label("tab ribbon");
                    });

                    // main body
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_max_height(200.);

                            ui.label("main body");
                        });
                    });
                });
            });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            cue_list_ui(ui, &mut self.state.project);
        });
    }
}

fn cue_list_ui(ui: &mut egui::Ui, project: &Project) -> () {
    let scroll_height = ui.available_height();
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .min_scrolled_height(0.0)
        .max_scroll_height(scroll_height)
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Q"));
            });
            header.col(|ui| {
                ui.strong("Type");
            });
            header.col(|ui| {
                ui.strong("Name");
            });
        })
        .body(|mut body| {
            for cue in &project.cues.list {
                body.row(18.0, |mut row| {
                    cue_row_ui(&mut row, cue);
                });
            }
        });
}

fn cue_row_ui(row: &mut TableRow, cue: &Cue) {
    row.col(|ui| {
        ui.label(RichText::new(format!("{}", cue.id)).text_style(egui::TextStyle::Monospace));
    });
    row.col(|ui| {
        ui.label("TBD");
    });
    row.col(|ui| {
        ui.label(cue.name.clone());
    });
}
