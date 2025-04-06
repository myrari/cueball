use std::cell::RefCell;

use rodio::{OutputStream, OutputStreamHandle};

thread_local!(
    pub static AUDIO_MANAGER: RefCell<Option<AudioManager>> = RefCell::new(None)
);

pub fn init() -> Result<(), anyhow::Error> {
    let (_stream, handle) = OutputStream::try_default()?;

    AUDIO_MANAGER.with(|mgr| {
        mgr.replace(Some(AudioManager { _stream, handle }));
    });

    Ok(())
}

pub struct AudioManager {
    _stream: OutputStream,
    pub handle: OutputStreamHandle,
}
