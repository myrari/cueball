use cueball::RemarkCue;

fn main() -> eframe::Result {
    env_logger::init();

    let mut state = cueball::AppState {
        project: cueball::Project::default(),
    };

    for i in 0..31 {
        state
            .project
            .cues
            .add(RemarkCue {
                id: i.to_string(),
                name: format!("Cue #{:04}", i),
                notes: "".to_string(),
            })
            .unwrap();
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
