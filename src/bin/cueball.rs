use cueball::app::{AppState, CueballApp, Project};

fn main() -> eframe::Result {
    env_logger::init();

    let state = AppState {
        project: Project::default(),
    };

    eframe::run_native(
        "cueball",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(CueballApp::new(cc, state)))),
    )
}
