pub mod inspector;

pub use inspector::AudioCueInspector;
use serde::{Deserialize, Serialize};

use std::{fs::File, io::BufReader, path::PathBuf};

use crate::{
    cues::{AudioCue, BonkCue, RemarkCue},
    Cue, MultitypeCue, Project,
};
use anyhow::anyhow;
use egui::{RichText, TextStyle};
use egui_extras::{Column, TableBuilder};
use inspector::get_cue_inspector;
use log::{debug, error};
use rfd::FileDialog;
const CUE_ID_WIDTH_PX: f32 = 50.;

#[derive(Serialize, Deserialize)]
pub struct CueballApp {
    state: AppState,
}

impl Default for CueballApp {
    fn default() -> Self {
        CueballApp {
            state: AppState {
                project: Project::default(),
                selected_cue: None,
                hovered_cue: None,
                dragged_cue: None,
                inspector_panel: InspectorPanel::default(),
            },
        }
    }
}

impl CueballApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // persistence
        if let Some(storage) = cc.storage {
            let mut stored: CueballApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            stored.state.project.cues.init_cues();
            return stored;
        }

        Default::default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub project: Project,

    selected_cue: Option<usize>,

    #[serde(skip)]
    dragged_cue: Option<usize>,
    #[serde(skip)]
    hovered_cue: Option<usize>,
    #[serde(skip)]
    inspector_panel: InspectorPanel,
}

impl AppState {
    fn select_cue(&mut self, new_cue_index: usize) -> Option<&MultitypeCue> {
        if new_cue_index < self.project.cues.list.len() {
            let new_cue = &self.project.cues.list[new_cue_index];
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

impl eframe::App for CueballApp {
    // save state on shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // paint frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // program-wide keyboard shortcuts
        ctx.input(|inp| {
            if inp.modifiers.command {
                // control-s for save(s)
                if inp.key_pressed(egui::Key::S) {
                    if inp.modifiers.shift {
                        // save-as
                        self.state.project.path = None;
                    }
                    match save_project(&self.state.project) {
                        Ok(path) => {
                            self.state.project.path = Some(path);
                        }
                        Err(err) => {
                            error!("Failed to save project: {}", err)
                        }
                    }
                }

                if inp.key_pressed(egui::Key::O) {
                    if inp.modifiers.ctrl && inp.key_pressed(egui::Key::O) {
                        match open_project() {
                            Ok(new_project) => {
                                self.state.project = new_project;
                            }
                            Err(err) => {
                                error!("Failed to open project: {}", err);
                            }
                        }
                    }
                }
            }
        });

        // top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // menu bar
            egui::menu::bar(ui, |ui| {
                // file menu and spacer
                ui.menu_button("File", |ui| {
                    // theme widget
                    egui::widgets::global_theme_preference_buttons(ui);

                    // save & save as buttons
                    if ui.button("Save").clicked() {
                        match save_project(&self.state.project) {
                            Ok(path) => {
                                self.state.project.path = Some(path);
                            }
                            Err(err) => {
                                error!("Failed to save project: {}", err)
                            }
                        }
                    }
                    // save as the same as save, but reset project path
                    if ui.button("Save As").clicked() {
                        self.state.project.path = None;
                        match save_project(&self.state.project) {
                            Ok(path) => {
                                self.state.project.path = Some(path);
                            }
                            Err(err) => {
                                error!("Failed to save project: {}", err)
                            }
                        }
                    }

                    // open button
                    if ui.button("Open").clicked() {
                        match open_project() {
                            Ok(new_project) => {
                                self.state.project = new_project;
                            }
                            Err(err) => {
                                error!("Failed to open project: {}", err);
                            }
                        }
                    }

                    // quit button
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // cues menu
                ui.menu_button("Cues", |ui| {
                    if ui.button("Audio").clicked() {
                        if let Ok(i) =
                            self.state
                                .project
                                .cues
                                .add(MultitypeCue::Audio(AudioCue::with_id(
                                    self.state.project.cues.get_new_cue_id().to_string(),
                                )))
                        {
                            self.state.select_cue(i);
                        }
                    }
                    if ui.button("Remark").clicked() {
                        if let Ok(i) =
                            self.state
                                .project
                                .cues
                                .add(MultitypeCue::Remark(RemarkCue::with_id(
                                    self.state.project.cues.get_new_cue_id().to_string(),
                                )))
                        {
                            self.state.select_cue(i);
                        }
                    }
                    if ui.button("Bonk").clicked() {
                        if let Ok(i) =
                            self.state
                                .project
                                .cues
                                .add(MultitypeCue::Bonk(BonkCue::with_id(
                                    self.state.project.cues.get_new_cue_id().to_string(),
                                )))
                        {
                            self.state.select_cue(i);
                        }
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("inspector_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.set_min_height(216.);
                if let Some(cue_index) = self.state.selected_cue {
                    ui.vertical(|ui| {
                        // tab ribbon
                        ui.horizontal(|ui| {
                            ui.set_height(16.);

                            // add buttons for each tab
                            ui.selectable_value(
                                &mut self.state.inspector_panel.selected_tab,
                                InspectorPanelTabs::Basics,
                                "Basics",
                            );
                            let cue = &mut self.state.project.cues.list[cue_index];
                            if let Some(cue_inspector) = get_cue_inspector(cue) {
                                if cue_inspector.time_and_loops() {
                                    ui.selectable_value(
                                        &mut self.state.inspector_panel.selected_tab,
                                        InspectorPanelTabs::TimeLoops,
                                        "Time & Loops",
                                    );
                                }
                            }
                        });

                        // main body
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            inspector_panel_body(ui, &mut self.state);
                        });
                    });
                } else {
                    ui.heading("Select a cue to edit it.");
                }
            });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            cue_list_ui(ui, &mut self.state);
        });
    }
}

fn open_project() -> Result<Project, anyhow::Error> {
    let path = match FileDialog::new()
        .add_filter("cueball", &["cueball", "cbp"])
        .pick_file()
    {
        None => {
            return Err(anyhow!("No file path selected!"));
        }
        Some(path) => path,
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut project: Project = serde_json::from_reader(reader)?;

    project.cues.init_cues();

    Ok(project)
}

fn save_project(project: &Project) -> Result<PathBuf, anyhow::Error> {
    match &project.path {
        None => {
            // pick new save path
            match FileDialog::new()
                .set_file_name(project.name.clone())
                .add_filter("cueball", &["cueball", "cbp"])
                .save_file()
            {
                None => Err(anyhow!("No file save path selected!")),
                Some(path) => {
                    let file = File::create(&path)?;
                    serde_json::to_writer(file, project)?;
                    Ok(path)
                }
            }
        }
        Some(path) => {
            // save path already set
            let file = File::create(&path)?;
            serde_json::to_writer(file, project)?;
            Ok(path.to_path_buf())
        }
    }
}

fn inspector_panel_body(ui: &mut egui::Ui, state: &mut AppState) {
    //let cue = &mut project.cues.list[project.selected_cue.unwrap()];
    let cue = match state.selected_cue {
        Some(cue_index) => &mut state.project.cues.list[cue_index],
        None => return,
    };
    ui.vertical(|ui| {
        //ui.set_min_height(200.);
        ui.set_width(ui.available_width());
        match state.inspector_panel.selected_tab {
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
                            egui::TextEdit::singleline(&mut state.inspector_panel.id_buf)
                                .font(TextStyle::Monospace)
                                .desired_width(CUE_ID_WIDTH_PX),
                        );
                        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            cue.set_id(state.inspector_panel.id_buf.as_str());
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
                if let Some(mut cue_inspector) = get_cue_inspector(cue) {
                    cue_inspector.basics(ui);
                }
            }
            InspectorPanelTabs::TimeLoops => {
                if let Some(mut cue_inspector) = get_cue_inspector(cue) {
                    cue_inspector.time_and_loops_fn(ui);
                }
            }
        }
    });
}

fn cue_list_ui(ui: &mut egui::Ui, state: &mut AppState) {
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
                if let Some(i) = state.selected_cue {
                    if inp.key_pressed(egui::Key::Home) {
                        state.select_cue(0);
                    }
                    if inp.key_pressed(egui::Key::ArrowDown) {
                        state.select_cue(i + 1);
                    }
                    if inp.key_pressed(egui::Key::ArrowUp) && i != 0 {
                        state.select_cue(i - 1);
                    }
                    if inp.key_pressed(egui::Key::End) && state.project.cues.list.len() != 0 {
                        state.select_cue(state.project.cues.list.len() - 1);
                    }
                    if inp.key_pressed(egui::Key::Space) && focus.is_none() {
                        handle_go(state);
                    }
                    if inp.pointer.primary_released() {
                        if let Some(h) = state.hovered_cue {
                            if let Some(d) = state.dragged_cue {
                                state.project.cues.move_cue(d, h);
                                state.select_cue(h);
                            }
                        }
                    }
                }
            });

            let mut hovered = false;
            let mut dragged = false;
            body.rows(18.0, state.project.cues.list.len(), |mut row| {
                let i = row.index();
                let this_selected = Some(i) == state.selected_cue;
                let cue = &state.project.cues.list[i];
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
                        state.selected_cue = None;
                    } else {
                        state.select_cue(i);
                    }
                }
                if response.dragged() {
                    state.dragged_cue = Some(i);
                    dragged = true;
                }
                if response.contains_pointer() {
                    state.hovered_cue = Some(i);
                    hovered = true;
                }
            });

            if !dragged {
                state.dragged_cue = None;
            }
            if !hovered {
                state.hovered_cue = None;
            }
        });
}

fn handle_go(state: &mut AppState) {
    // get current cue
    let cue_index = match state.selected_cue {
        Some(i) => i,
        None => {
            debug!("No cue selected for Go");
            return;
        }
    };

    // immutably get cue for next cue index
    let cue = &state.project.cues.list[cue_index];
    let next_cue_index = cue_index + cue.next_offset();

    let cue_mut = &mut state.project.cues.list[cue_index];

    // play current cue
    cue_mut.go();

    // advance playhead
    if let None = state.select_cue(next_cue_index) {
        state.selected_cue = None;
    }
}
