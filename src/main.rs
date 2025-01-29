fn main() -> eframe::Result {
    env_logger::init();

    let mut state = cueball::AppState {
        project: cueball::Project::default(),
    };

    eframe::run_native(
        "cueball",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(cueball::CueballApp::new(cc, state)))),
    )
}
