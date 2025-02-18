use crate::data::{Cue, CueList};
use egui::{TextStyle, RichText};
use egui_extras::{Column, TableBuilder};

const CUE_ID_WIDTH_PX: f32 = 50.;

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

impl Project {
    fn select_cue(&mut self, new_cue_index: usize) -> Option<&Box<dyn Cue>> {
        if new_cue_index < self.cues.list.len() {
            let new_cue = &self.cues.list[new_cue_index];
            self.selected_cue = Some(new_cue_index);
            self.inspector_panel.id_buf = new_cue.get_id();
            Some(new_cue)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct InspectorPanel {
    selected_tab: InspectorPanelTabs,
    id_buf: String
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            selected_tab: InspectorPanelTabs::Basics,
            id_buf: String::new()
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
                        ui.set_width(80.);
                        ui.label("ID:");
                        let resp = ui.add(egui::TextEdit::singleline(
                            &mut project.inspector_panel.id_buf)
                            .font(TextStyle::Monospace)
                            .desired_width(CUE_ID_WIDTH_PX));
                        if resp.lost_focus() &&
                                ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            cue.set_id(project.inspector_panel.id_buf.as_str());
                        }
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
                ui.set_min_width(CUE_ID_WIDTH_PX);
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
                        project.select_cue(0);
                    }
                    if inp.key_pressed(egui::Key::ArrowDown) {
                        project.select_cue(i + 1);
                    }
                    if inp.key_pressed(egui::Key::ArrowUp) && i != 0 {
                        project.select_cue(i - 1);
                    }
                    if inp.key_pressed(egui::Key::End) && project.cues.list.len() != 0 {
                        project.select_cue(project.cues.list.len() - 1);
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
                });
                row.col(|ui| {
                    ui.label(cue.type_str_short());
                });
                row.col(|ui| {
                    ui.label(cue.get_name());
                });
                if row.response().clicked() {
                    if this_selected {
                        project.selected_cue = None;
                    } else {
                        project.select_cue(i);
                    }
                }
            });
        });
}

fn handle_go(_project: &mut Project) {
    // Actual functionality to be added
    println!("Go!");
}
