fn main() -> eframe::Result {
    env_logger::init();

    let mut state = cueball::AppState {
        project: cueball::Project::default(),
    };

    for _ in 0..24 {
        state
            .project
            .cues
            .add(cueball::CueType::Message(String::from("hi")));
    }

    eframe::run_native(
        "cueball",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(cueball::CueballApp::new(cc, state)))),
    )
}
