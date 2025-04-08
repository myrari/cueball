use anyhow::anyhow;
use cueball::{app::CueballApp, audio};

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    audio::init()?;

    match eframe::run_native(
        "cueball",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(CueballApp::new(cc)))),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("{}", err)),
    }
}
