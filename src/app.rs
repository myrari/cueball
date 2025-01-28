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
            ui.heading("Cueball");

            cue_list_ui(ui, &mut self.state.project);
        });
    }
}

fn cue_list_ui(ui: &mut egui::Ui, project: &mut Project) -> () {
    const ROW_HEIGHT: f32 = 24.;

    egui::ScrollArea::vertical().show(ui, |ui| {
        // header
        //ui.horizontal(|ui| {
        //    // cue id column
        //    ui.horizontal(|ui| {
        //        ui.set_width(16.);
        //        ui.label("Cue #");
        //    });
        //
        //    ui.separator();
        //
        //    ui.label("Cue");
        //});

        for cue in &project.cues.list {
            ui.horizontal(|ui| {
                ui.set_height(ROW_HEIGHT);

                // cue number
                ui.horizontal(|ui| {
                    ui.set_width(16.);

                    ui.label(cue.id.to_string());
                });

                ui.separator();

                // cue body
                ui.horizontal(|ui| {
                    let mut selected_cue = project.selected_cue;
                    cue_body_ui(ui, cue, &mut selected_cue);
                    project.selected_cue = selected_cue;
                });
            });

            ui.separator();
        }
    });

    //ui.horizontal(|ui| {
    //    // cue id column
    //    ui.vertical(|ui| {
    //        ui.set_width(16.);
    //
    //        for cue in &project.cues.list {
    //            ui.horizontal(|ui| {
    //                ui.set_height(ROW_HEIGHT);
    //
    //                ui.label(cue.id.to_string());
    //            });
    //            ui.separator();
    //        }
    //    });
    //
    //    ui.separator();
    //
    //    // cue column
    //    ui.vertical(|ui| {
    //        for cue in &project.cues.list {
    //            ui.horizontal(|ui| {
    //                ui.set_height(ROW_HEIGHT);
    //
    //                let mut selected_cue: Option<u64> = project.selected_cue;
    //                cue_body_ui(ui, cue, &mut selected_cue);
    //                project.selected_cue = selected_cue;
    //            });
    //            ui.separator();
    //        }
    //    });
    //});
}

fn cue_body_ui(ui: &mut egui::Ui, cue: &Cue, selected_cue: &mut Option<u64>) {
    match &cue.cue_type {
        CueType::Message(_msg) => {
            // wip icon
            ui.label("(M)");
        }
        CueType::Process => {
            // wip icon
            ui.label("(P)");
        }
    };
    ui.selectable_value(selected_cue, Some(cue.id), cue.name.clone());
}
