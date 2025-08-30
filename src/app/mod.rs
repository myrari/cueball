pub mod inspector;

pub use inspector::AudioCueInspector;
use inspector::{get_cue_inspector, InspectorPanelTabs};

use crate::{
    cues::{AudioCue, BonkCue, RemarkCue},
    Cue, MultitypeCue, Project,
};

use anyhow::anyhow;
use egui::{Color32, Rect, RichText, Stroke, TextStyle};
use egui_extras::{Column, TableBuilder};
use log::{debug, error};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::PathBuf};
const CUE_ID_WIDTH_PX: f32 = 50.;

#[derive(Serialize, Deserialize)]
pub struct CueballApp {
    #[serde(skip)]
    state: AppState,

    project_path: Option<PathBuf>,
}

impl Default for CueballApp {
    fn default() -> Self {
        CueballApp {
            state: AppState::default(),
            project_path: None,
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
            if let Some(ref project_path) = stored.project_path {
                match open_project(project_path.clone()) {
                    Ok(new_project) => {
                        stored.state.project = new_project;
                    }
                    Err(err) => {
                        error!("Failed to open project: {}", err);
                    }
                }
            }
            return stored;
        }

        Default::default()
    }

    fn set_project_path(&mut self, path: Option<PathBuf>) -> () {
        self.project_path = path.clone();
        self.state.project.path = path.clone();
    }
}

#[derive(Debug)]
struct DebugSettings {
    disable_continue: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        DebugSettings {
            disable_continue: false,
        }
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

    #[serde(skip)]
    debug_settings: DebugSettings,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            project: Project::default(),
            selected_cue: None,
            hovered_cue: None,
            dragged_cue: None,
            inspector_panel: InspectorPanel::default(),
            debug_settings: DebugSettings::default(),
        }
    }
}

impl AppState {
    fn select_cue(&mut self, new_cue_index: usize) -> Option<&MultitypeCue> {
        if new_cue_index < self.project.cues.len() {
            let new_cue = &self.project.cues[new_cue_index];
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
                        self.set_project_path(None);
                    }
                    match save_project(&self.state.project) {
                        Ok(path) => {
                            self.set_project_path(Some(path));
                        }
                        Err(err) => {
                            error!("Failed to save project: {}", err)
                        }
                    }
                }

                if inp.key_pressed(egui::Key::O) {
                    if inp.modifiers.ctrl && inp.key_pressed(egui::Key::O) {
                        match FileDialog::new()
                            .add_filter("cueball", &["cueball", "cbp"])
                            .pick_file()
                        {
                            None => {
                                error!("No file path selected!");
                            }
                            Some(path) => match open_project(path) {
                                Ok(new_project) => {
                                    self.state.project = new_project;
                                }
                                Err(err) => {
                                    error!("Failed to open project: {}", err);
                                }
                            },
                        };
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
                                self.set_project_path(Some(path));
                            }
                            Err(err) => {
                                error!("Failed to save project: {}", err)
                            }
                        }
                    }

                    // save as the same as save, but reset project path
                    if ui.button("Save As").clicked() {
                        self.set_project_path(None);
                        match save_project(&self.state.project) {
                            Ok(path) => {
                                self.set_project_path(Some(path));
                            }
                            Err(err) => {
                                error!("Failed to save project: {}", err)
                            }
                        }
                    }

                    // open button
                    if ui.button("Open").clicked() {
                        match FileDialog::new()
                            .add_filter("cueball", &["cueball", "cbp"])
                            .pick_file()
                        {
                            None => {
                                error!("No file path selected!");
                            }
                            Some(path) => match open_project(path) {
                                Ok(new_project) => {
                                    self.state.project = new_project;
                                }
                                Err(err) => {
                                    error!("Failed to open project: {}", err);
                                }
                            },
                        };
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

                // ui.with_layout(
                //     egui::Layout::top_down_justified(egui::Align::Center),
                //     |ui| ui.label(RichText::new(self.state.project.name.clone()).strong()),
                // );
                // ui.horizontal(|ui| {
                //     ui.label(RichText::new(self.state.project.name.clone()).strong());
                // });
                // ui.horizontal(|ui| {
                //     ui.set_min_width(32.);
                //     // ui.separator();
                //     ui.label("Debug Settings:");
                //     ui.toggle_value(
                //         &mut self.state.debug_settings.disable_continue,
                //         "Disable Continue",
                //     );
                // });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                    ui.toggle_value(
                        &mut self.state.debug_settings.disable_continue,
                        "Disable Continue",
                    );
                    ui.label("Debug Settings:");
                    ui.add_sized(
                        ui.available_size(),
                        egui::Label::new(RichText::new(self.state.project.name.clone()).strong()),
                    )
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
                            let cue = &mut self.state.project.cues[cue_index];
                            if let Some(cue_inspector) = get_cue_inspector(cue) {
                                for (tab, name) in InspectorPanelTabs::ITER {
                                    if cue_inspector.has_tab(&tab) {
                                        ui.selectable_value(
                                            &mut self.state.inspector_panel.selected_tab,
                                            tab,
                                            name,
                                        );
                                    }
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

fn open_project(path: PathBuf) -> Result<Project, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut project: Project = serde_json::from_reader(reader)?;

    project.cues.init_cues();

    debug!("Loaded project {}", project.name);

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
                    debug!("Saved project {} to {}", project.name, path.display());
                    Ok(path)
                }
            }
        }
        Some(path) => {
            // save path already set
            let file = File::create(&path)?;
            serde_json::to_writer(file, project)?;
            debug!("Saved project {} to {}", project.name, path.display());
            Ok(path.to_path_buf())
        }
    }
}

fn inspector_panel_body(ui: &mut egui::Ui, state: &mut AppState) {
    //let cue = &mut project.cues.list[project.selected_cue.unwrap()];
    let cue = match state.selected_cue {
        Some(cue_index) => &mut state.project.cues[cue_index],
        None => return,
    };
    ui.vertical(|ui| {
        //ui.set_min_height(200.);
        ui.set_width(ui.available_width());

        // special handling for basics tab
        if state.inspector_panel.selected_tab == InspectorPanelTabs::Basics {
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
                cue_inspector.draw_tab(ui, &InspectorPanelTabs::Basics);
            }
        } else {
            if let Some(mut cue_inspector) = get_cue_inspector(cue) {
                cue_inspector.draw_tab(ui, &state.inspector_panel.selected_tab);
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
        .column(Column::remainder())
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
            header.col(|ui| {
                ui.strong("Duration");
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
                    if inp.key_pressed(egui::Key::End) && state.project.cues.len() != 0 {
                        state.select_cue(state.project.cues.len() - 1);
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
            body.rows(18.0, state.project.cues.len(), |mut row| {
                let i = row.index();
                let this_selected = Some(i) == state.selected_cue;
                let cue = &state.project.cues[i];
                row.set_selected(this_selected);

                let mut this_clicked = false;
                let mut this_dragged = false;
                let mut this_hovered = false;

                // cue id
                row.col(|ui| {
                    let resp = ui
                        .label(RichText::new(cue.get_id()).text_style(egui::TextStyle::Monospace));
                    this_clicked |= resp.clicked();
                    this_dragged |= resp.dragged();
                    this_hovered |= resp.contains_pointer();
                });
                // cue type
                row.col(|ui| {
                    let resp = ui.label(cue.type_str_short());
                    this_clicked |= resp.clicked();
                    this_dragged |= resp.dragged();
                    this_hovered |= resp.contains_pointer();
                });
                // cue name
                row.col(|ui| {
                    let resp = ui.label(cue.get_name());
                    this_clicked |= resp.clicked();
                    this_dragged |= resp.dragged();
                    this_hovered |= resp.contains_pointer();
                });

                // times column
                row.col(|ui| {
                    ui.set_max_width(64.);
                    if let Some(len) = cue.length() {
                        let rect = ui.available_rect_before_wrap();
                        // let painter = ui.painter_at(rect);
                        let painter = ui.painter();
                        painter.rect_stroke(
                            rect,
                            0.,
                            Stroke::new(2., Color32::from_rgb(0, 200, 0)),
                        );
                        if let Some(el) = cue.elapsed() {
                            let el_width = el / len * rect.width();
                            painter.rect_filled(
                                Rect {
                                    min: rect.min,
                                    max: egui::Pos2 {
                                        x: rect.min.x + el_width,
                                        y: rect.max.y,
                                    },
                                },
                                0.,
                                Color32::from_rgba_unmultiplied(0, 128, 0, 64),
                            );
                            ui.ctx().request_repaint();
                            ui.label(format!("{:.3}", el));
                        } else {
                            ui.label(format!("{:.3}", len));
                        }
                    } else {
                        ui.weak(format!("{:.3}", 0.));
                    }
                });

                let resp = row.response();
                this_clicked |= resp.clicked();
                this_dragged |= resp.dragged();
                this_hovered |= resp.contains_pointer();

                if this_clicked {
                    if this_selected {
                        state.selected_cue = None;
                    } else {
                        state.select_cue(i);
                    }
                }
                if this_dragged {
                    state.dragged_cue = Some(i);
                    dragged = true;
                }
                if this_hovered {
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
    let cue = &state.project.cues[cue_index];
    let next_cue_index = cue_index
        + if state.debug_settings.disable_continue {
            0
        } else {
            cue.next_offset()
        };

    let cue_mut = &mut state.project.cues[cue_index];

    // play current cue
    cue_mut.go();

    // advance playhead
    if let None = state.select_cue(next_cue_index) {
        state.selected_cue = None;
    }
}
