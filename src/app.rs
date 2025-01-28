use std::collections::HashSet;

#[derive(Debug)]
pub struct CueballApp {
    state: AppState,
}

#[derive(Debug)]
struct AppState {
    project: Project,
}

#[derive(Debug)]
struct Project {
    name: String,

    cues: CueList,
}

#[derive(Debug)]
struct CueList {
    list: Vec<Cue>,
}

impl CueList {
    fn new() -> Self {
        Self { list: vec![] }
    }

    fn add(&mut self, cue_type: CueType) {
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
enum CueType {
    Message(String),
    Process,
}

impl Default for CueballApp {
    fn default() -> Self {
        let mut app = CueballApp {
            state: AppState {
                project: Project {
                    name: String::from("Untitled.cbp"),
                    cues: CueList::new(),
                },
            },
        };

        app.state.project.cues.add(CueType::Message(String::from("hi")));
        app.state.project.cues.add(CueType::Message(String::from("there")));
        app.state.project.cues.add(CueType::Message(String::from("tucker")));

        app
    }
}

impl CueballApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // do nothing here for now
        Default::default()
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
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                // theme widget
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cueball");

            cue_list_ui(ui, &self.state.project);
        });
    }
}

fn cue_list_ui(ui: &mut egui::Ui, project: &Project) -> () {
    ui.vertical(|ui| {
        for cue in &project.cues.list {
            ui.label(cue.name.clone());
        }
    });
}
