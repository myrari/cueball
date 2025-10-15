use std::cell::RefCell;

use rodio::{OutputStream, OutputStreamBuilder};

thread_local!(
    pub static AUDIO_MANAGER: RefCell<Option<AudioManager>> = RefCell::new(None)
);

pub fn init() -> Result<(), anyhow::Error> {
    let stream = OutputStreamBuilder::open_default_stream()?;

    AUDIO_MANAGER.with(|mgr| {
        mgr.replace(Some(AudioManager { stream }));
    });

    Ok(())
}

pub struct AudioManager {
    pub stream: OutputStream,
}
