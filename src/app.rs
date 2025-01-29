use egui::RichText;
use egui_extras::{Column, TableBuilder};

use crate::CueList;

pub struct CueballApp {
    state: AppState,
}

impl Default for CueballApp {
    fn default() -> Self {
        CueballApp {
            state: AppState {
                project: Project::default(),
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

pub struct AppState {
    pub project: Project,
}

pub struct Project {
    pub name: String,

    pub cues: CueList,

    selected_cue: Option<usize>,
    inspector_panel: InspectorPanel,
}

#[derive(Debug)]
struct InspectorPanel {
    selected_tab: InspectorPanelTabs,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            selected_tab: InspectorPanelTabs::Basics,
        }
    }
}

#[derive(Debug, PartialEq)]
enum InspectorPanelTabs {
    Basics,
    TimeLoops,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("untitled.cueball"),
            cues: CueList::new(),
            selected_cue: None,
            inspector_panel: InspectorPanel::default(),
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
            ui.label("test");
        });

        egui::TopBottomPanel::bottom("inspector_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.set_min_height(216.);
                if let Some(_cue_index) = self.state.project.selected_cue {
                    ui.vertical(|ui| {
                        // tab ribbon
                        ui.horizontal(|ui| {
                            ui.set_height(16.);

                            // add buttons for each tab
                            ui.selectable_value(
                                &mut self.state.project.inspector_panel.selected_tab,
                                InspectorPanelTabs::Basics,
                                "Basics",
                            );
                            ui.selectable_value(
                                &mut self.state.project.inspector_panel.selected_tab,
                                InspectorPanelTabs::TimeLoops,
                                "Time & Loops",
                            );
                        });

                        // main body
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            inspector_panel_body(ui, &mut self.state.project);
                        });
                    });
                } else {
                    ui.heading("Select a cue to edit it.");
                }
            });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            cue_list_ui(ui, &mut self.state.project);
        });
    }
}

fn inspector_panel_body(ui: &mut egui::Ui, project: &mut Project) {
    ui.horizontal(|ui| {
        //ui.set_min_height(200.);
        ui.set_width(ui.available_width());
        match project.inspector_panel.selected_tab {
            InspectorPanelTabs::Basics => {
                let cue = &mut project.cues.list[project.selected_cue.unwrap()];
                // first row
                ui.horizontal(|ui| {
                    // cue number
                    ui.horizontal(|ui| {
                        ui.label(format!("Type: {}", cue.type_str_full()));
                    });
                    ui.horizontal(|ui| {
                        ui.label("ID:");
                        let mut cue_id = cue.get_id();
                        ui.text_edit_singleline(&mut cue_id);
                        cue.set_id(cue_id.as_str());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        let mut cue_name = cue.get_name();
                        ui.text_edit_singleline(&mut cue_name);
                        cue.set_name(cue_name.as_str());
                    });
                });
            }
            InspectorPanelTabs::TimeLoops => {
                ui.label("time & loops lol");
            }
        }
    });
}

fn cue_list_ui(ui: &mut egui::Ui, project: &mut Project) {
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
        .sense(egui::Sense::click())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Q");
            });
            header.col(|ui| {
                ui.strong("Type");
            });
            header.col(|ui| {
                ui.strong("Name");
            });
        })
        .body(|mut body| {
            body.ui_mut().input(|inp| {
                if let Some(i) = project.selected_cue {
                    if inp.key_pressed(egui::Key::Home) {
                        project.selected_cue = Some(0);
                    }
                    if inp.key_pressed(egui::Key::ArrowDown) && i + 1 != project.cues.list.len() {
                        project.selected_cue = Some(i + 1);
                    }
                    if inp.key_pressed(egui::Key::ArrowUp) && i != 0 {
                        project.selected_cue = Some(i - 1);
                    }
                    if inp.key_pressed(egui::Key::End) && project.cues.list.len() != 0 {
                        project.selected_cue = Some(project.cues.list.len() - 1);
                    }
                    if inp.key_pressed(egui::Key::Space) {
                        handle_go(project);
                    }
                }
            });
            body.rows(18.0, project.cues.list.len(), |mut row| {
                let i = row.index();
                let this_selected = Some(i) == project.selected_cue;
                let cue = &project.cues.list[i];
                row.set_selected(this_selected);
                row.col(|ui| {
                    ui.label(RichText::new(cue.get_id()).text_style(egui::TextStyle::Monospace));
                    //for cue in &project.cues.list {
                    //    body.row(18.0, |mut row| {
                    //        cue_row_ui(&mut row, cue, &mut project.selected_cue);
                });
                row.col(|ui| {
                    ui.label(cue.type_str_short());
                });
                row.col(|ui| {
                    ui.label(cue.get_name());
                });
                if row.response().clicked() {
                    if this_selected {
                        project.selected_cue = None
                    } else {
                        project.selected_cue = Some(i)
                    }
                }
            });
        });
}

fn handle_go(_project: &mut Project) {
    // Actual functionality to be added
    println!("Go!");
}

//fn cue_row_ui(row: &mut TableRow, cue: &Box<dyn Cue>, selected_cue: &mut Option<String>) {
//    if selected_cue.as_ref().is_some_and(|c| *c == cue.get_id()) {
//        // this cue is selected
//        row.set_selected(true);
//    }
//
//    row.col(|ui| {
//        ui.label(RichText::new(cue.get_id()).text_style(egui::TextStyle::Monospace));
//    });
//    row.col(|ui| {
//        ui.label(cue.type_str_short());
//    });
//    row.col(|ui| {
//        ui.label(cue.get_name());
//    });
//
//    if row.response().clicked() {
//        *selected_cue = Some(cue.get_id());
//    }
//}
