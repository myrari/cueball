pub struct CueballApp {
    state: AppState,
}

pub struct AppState {
    project: String,
}

impl Default for CueballApp {
    fn default() -> Self {
        Self {
            state: AppState {
                project: String::from("Untitled.cbp"),
            },
        }
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

            ui.label("text");
        });
    }
}
