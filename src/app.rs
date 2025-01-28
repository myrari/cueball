use std::collections::HashSet;
use egui_extras::{TableBuilder,Table,Column};
use egui::RichText;

#[derive(Debug)]
pub struct CueballApp {
    state: AppState,
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
            name: String::from("Untitled.cpb"),
            cues: CueList::new(),
            selected_cue: None,
        }
    }
}

#[derive(Debug)]
pub struct CueList {
    list: Vec<Cue>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, cue_type: CueType) {
        let cue = Cue {
            id: self.get_new_cue_id(),
            name: match &cue_type {
                CueType::Message(msg) => msg.clone(),
                CueType::Process => String::from("Process cue"),
            },
            cue_type,
        };
        self.list.push(cue);
    }

    fn get_new_cue_id(&self) -> u64 {
        let mut largest_id = 0;

        for cue in &self.list {
            if cue.id > largest_id {
                largest_id = cue.id;
            }
        }

        largest_id + 1
    }
}

#[derive(Debug)]
struct Cue {
    id: u64,
    name: String,

    cue_type: CueType,
}

#[derive(Debug)]
pub enum CueType {
    Message(String),
    Process,
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
            header.col(|ui| {ui.label(RichText::new("Q"));});
            header.col(|ui| {ui.strong("Type");});
            header.col(|ui| {ui.strong("Name");});
        })
        .body(|mut body| {
            for cue in &project.cues.list {
                body.row(18.0, |mut row| {
                    row.col(|ui| {ui.label("TBD");});
                    row.col(|ui| {ui.label("TBD");});
                    row.col(|ui| {ui.label(cue.name.clone());});
                });
            }
        });
}
