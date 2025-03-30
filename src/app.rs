use crate::{cue_imp::BonkCue, Cue, CueList, RemarkCue};
use egui::{RichText, TextStyle};
use egui_extras::{Column, TableBuilder};
use log::debug;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,

    pub cues: CueList,

    #[serde(skip)]
    selected_cue: Option<usize>,
    #[serde(skip)]
    dragged_cue: Option<usize>,
    #[serde(skip)]
    hovered_cue: Option<usize>,
    #[serde(skip)]
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
    id_buf: String,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            selected_tab: InspectorPanelTabs::Basics,
            id_buf: String::new(),
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
            hovered_cue: None,
            dragged_cue: None,
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
                // cues menu
                ui.menu_button("Cues", |ui| {
                    if ui.button("Remark").clicked() {
                        if let Ok(i) = self.state.project.cues.add(RemarkCue::with_id(
                            self.state.project.cues.get_new_cue_id().to_string(),
                        )) {
                            self.state.project.select_cue(i);
                        }
                    }
                    if ui.button("Bonk").clicked() {
                        if let Ok(i) = self.state.project.cues.add(BonkCue::with_id(
                            self.state.project.cues.get_new_cue_id().to_string(),
                        )) {
                            self.state.project.select_cue(i);
                        }
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("inspector_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.set_min_height(216.);
                if let Some(cue_index) = self.state.project.selected_cue {
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
                            let cue = &mut self.state.project.cues.list[cue_index];
                            if let Some(mut cue_inspector) = cue.inspector() {
                                if let Some(_) = cue_inspector.time_and_loops() {
                                    ui.selectable_value(
                                        &mut self.state.project.inspector_panel.selected_tab,
                                        InspectorPanelTabs::TimeLoops,
                                        "Time & Loops",
                                    );
                                }
                            }
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
    //let cue = &mut project.cues.list[project.selected_cue.unwrap()];
    let cue = match project.selected_cue {
        Some(cue_index) => &mut project.cues.list[cue_index],
        None => return,
    };
    ui.vertical(|ui| {
        //ui.set_min_height(200.);
        ui.set_width(ui.available_width());
        match project.inspector_panel.selected_tab {
            InspectorPanelTabs::Basics => {
                // first row, default things for all cues
                ui.horizontal(|ui| {
                    // cue number
                    ui.horizontal(|ui| {
                        ui.label(format!("Type: {}", cue.type_str_full()));
                    });
                    ui.horizontal(|ui| {
                        ui.set_width(80.);
                        ui.label("ID:");
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut project.inspector_panel.id_buf)
                                .font(TextStyle::Monospace)
                                .desired_width(CUE_ID_WIDTH_PX),
                        );
                        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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

                // second row, for things specifc to a type of cue
                if let Some(mut cue_inspector) = cue.inspector() {
                    cue_inspector.basics(ui);
                }
            }
            InspectorPanelTabs::TimeLoops => {
                if let Some(mut cue_inspector) = cue.inspector() {
                    if let Some(time_loops_func) = cue_inspector.time_and_loops() {
                        time_loops_func(ui);
                    }
                }
            }
        }
    });
}

fn cue_list_ui(ui: &mut egui::Ui, project: &mut Project) {
    let focus = ui.memory(|mem| mem.focused());

    let scroll_height = ui.available_height();
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .min_scrolled_height(0.0)
        .max_scroll_height(scroll_height)
        .drag_to_scroll(false)
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::remainder())
        //.sense(egui::Sense::click())
        .sense(egui::Sense::click_and_drag())
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
                    if inp.key_pressed(egui::Key::Space) && focus.is_none() {
                        handle_go(project);
                    }
                    if inp.pointer.primary_released() {
                        if let Some(h) = project.hovered_cue {
                            if let Some(d) = project.dragged_cue {
                                project.cues.move_cue(d, h);
                                project.select_cue(h);
                            }
                        }
                    }
                }
            });

            let mut hovered = false;
            let mut dragged = false;
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
                let response = row.response();
                if response.clicked() {
                    if this_selected {
                        project.selected_cue = None;
                    } else {
                        project.select_cue(i);
                    }
                }
                if response.dragged() {
                    project.dragged_cue = Some(i);
                    dragged = true;
                }
                if response.contains_pointer() {
                    project.hovered_cue = Some(i);
                    hovered = true;
                }
            });

            if !dragged {
                project.dragged_cue = None;
            }
            if !hovered {
                project.hovered_cue = None;
            }
        });
}

fn handle_go(project: &mut Project) {
    // get current cue
    let cue_index = match project.selected_cue {
        Some(i) => i,
        None => {
            debug!("No cue selected for Go");
            return;
        }
    };

    // immutably get cue for next cue index
    let cue = &project.cues.list[cue_index];
    let next_cue_index = cue_index + cue.next_offset();

    let cue_mut = &mut project.cues.list[cue_index];

    // play current cue
    cue_mut.go();

    // advance playhead
    if let None = project.select_cue(next_cue_index) {
        project.selected_cue = None;
    }
}
