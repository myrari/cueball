use cueball::app::CueballApp;

fn main() -> eframe::Result {
    env_logger::init();

    eframe::run_native(
        "cueball",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(CueballApp::default()))),
    )
}
