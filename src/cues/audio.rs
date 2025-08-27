use std::{fmt::Debug, fs::File, io::BufReader, time::Duration};

use crate::audio;
use anyhow::anyhow;
use log::{debug, error, warn};
use mlua::prelude::*;
use rodio::{Decoder, Sink, Source};
use serde::{Deserialize, Serialize};

use super::{add_common_lua_fields, add_common_lua_methods, Cue, CueRunning, CueTime};

#[derive(Serialize, Deserialize)]
pub struct AudioCue {
    pub id: String,
    pub name: String,

    pub file_path: String,

    pub start: f32,
    pub end: f32,

    #[serde(default = "default_volume")]
    volume: f32,

    #[serde(skip)]
    pub sink: Option<Box<Sink>>,
    #[serde(skip)]
    pub duration: Option<f32>,
}

fn default_volume() -> f32 {
    1.0
}

impl AudioCue {
    pub fn with_id(id: String) -> Self {
        Self {
            id,
            name: "New audio cue".into(),
            file_path: "".into(),
            start: 0.,
            end: 0.,
            volume: 1.,
            sink: None,
            duration: None,
        }
    }

    fn play_audio(&mut self) -> Result<(), anyhow::Error> {
        if let Some(sink) = &self.sink {
            let file = BufReader::new(File::open(self.file_path.clone())?);

            let source = Decoder::new(file)?;
            let buffer = source.buffered();

            let sample_rate = buffer.sample_rate();
            let num_samples = buffer.clone().count();
            let duration =
                Duration::from_secs_f32((num_samples as f32) / (2. * sample_rate as f32));

            // apply start and end offsets
            let trimmed_source = buffer
                .take_duration(duration - Duration::from_secs_f32(self.end))
                .skip_duration(Duration::from_secs_f32(self.start));

            if !sink.empty() {
                sink.clear();
            }

            sink.set_volume(self.volume);

            sink.append(trimmed_source);
            sink.play();

            Ok(())
        } else {
            Err(anyhow!("Not initialized!"))
        }
    }

    fn init_duration(&mut self) -> Result<(), anyhow::Error> {
        let file = BufReader::new(File::open(self.file_path.clone())?);

        let source = Decoder::new(file)?;

        let sample_rate = source.sample_rate();
        let num_samples = source.count();
        let raw_duration = (num_samples as f32) / (2. * sample_rate as f32);
        self.duration = Some(raw_duration - (self.start + self.end));
        Ok(())
    }

    pub fn set_start(&mut self, v: f32) -> Result<(), anyhow::Error> {
        Ok(match self.duration {
            Some(d) => {
                let diff = self.start - v;
                self.duration = Some(d + diff);
                self.start = v;
            }
            None => {
                self.start = v;
                self.init_duration()?;
            }
        })
    }

    pub fn set_end(&mut self, v: f32) -> Result<(), anyhow::Error> {
        Ok(match self.duration {
            Some(d) => {
                let diff = self.end - v;
                self.duration = Some(d + diff);
                self.end = v;
            }
            None => {
                self.end = v;
                self.init_duration()?;
            }
        })
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn set_volume(&mut self, v: f32) -> Result<(), anyhow::Error> {
        if let Some(sink) = &self.sink {
            self.volume = v;
            sink.set_volume(self.volume);

            Ok(())
        } else {
            Err(anyhow!("Not initialized!"))
        }
    }
}

impl PartialEq for AudioCue {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.name.eq(&other.name) && self.file_path.eq(&other.file_path)
    }
}

impl Eq for AudioCue {}

impl Clone for AudioCue {
    fn clone(&self) -> Self {
        audio::AUDIO_MANAGER.with_borrow(|am| {
            if let Some(am) = am {
                Self {
                    id: self.id.clone(),
                    name: self.name.clone(),
                    file_path: self.file_path.clone(),
                    start: self.start,
                    end: self.end,
                    volume: self.volume,
                    sink: Some(Box::new(
                        Sink::try_new(&am.handle).expect("Failed to create sink for audio cue"),
                    )),
                    duration: self.duration,
                }
            } else {
                Self {
                    id: self.id.clone(),
                    name: self.name.clone(),
                    file_path: self.file_path.clone(),
                    sink: None,
                    start: self.start,
                    end: self.end,
                    volume: self.volume,
                    duration: self.duration,
                }
            }
        })
    }
}

impl Debug for AudioCue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioCue")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("file_path", &self.file_path)
            .field("sink", &self.sink.is_some())
            .finish()
    }
}

#[typetag::serde]
impl Cue for AudioCue {
    fn init(&mut self) -> () {
        // info!("Init audio cue {}", self.id);
        // initialize the sink for this cue
        if self.sink.is_some() {
            debug!("Audio cue {} already initted!", self.id)
        }
        audio::AUDIO_MANAGER.with_borrow(|am| match am {
            Some(am) => match Sink::try_new(&am.handle) {
                Ok(sink) => self.sink = Some(Box::new(sink)),
                Err(err) => {
                    error!(
                        "Could not init audio cue {}, sink creation error: {}",
                        self.id, err
                    )
                }
            },
            None => {
                error!(
                    "Could not init audio cue {}, AudioManager not intialized!",
                    self.id
                )
            }
        });
        // intialize duration, read from file
        match self.init_duration() {
            Err(err) => {
                if let Some(_) = err.downcast_ref::<std::io::Error>() {
                    warn!(
                        "Tried to init audio cue {} with invalid audio file",
                        self.id
                    )
                } else {
                    error!(
                        "Could not init audio cue {}, duration err: {}",
                        self.id, err
                    )
                }
            }
            Ok(_) => {}
        };
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn set_id(&mut self, new_id: &str) -> () {
        self.id = new_id.to_string();
    }
    fn set_name(&mut self, new_name: &str) -> () {
        self.name = new_name.to_string();
    }
    fn type_str_full(&self) -> String {
        "Audio".to_string()
    }
    fn type_str_short(&self) -> String {
        "Aud".to_string()
    }

    fn go(&mut self) -> () {
        if let Err(err) = self.play_audio() {
            error!("Error playing audio cue {}: {}", self.id, err);
        }
    }

    fn running(&self) -> CueRunning {
        if let Some(sink) = &self.sink {
            if sink.empty() {
                CueRunning::Stopped
            } else if sink.is_paused() {
                CueRunning::Paused
            } else {
                CueRunning::Running
            }
        } else {
            CueRunning::Stopped
        }
    }

    fn stop(&mut self) -> () {
        if let Some(sink) = &self.sink {
            sink.clear();
        }
    }

    fn set_paused(&mut self, pu: bool) -> () {
        if let Some(sink) = &self.sink {
            if pu {
                sink.pause();
            } else {
                sink.play();
            }
        }
    }

    fn length(&self) -> Option<CueTime> {
        self.duration
    }

    fn elapsed(&self) -> Option<CueTime> {
        if let Some(sink) = &self.sink {
            if !sink.empty() {
                Some(sink.get_pos().as_secs_f32())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn remaining(&self) -> Option<CueTime> {
        if let Some(sink) = &self.sink {
            if let Some(dur) = self.duration {
                Some(dur - sink.get_pos().as_secs_f32())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl LuaUserData for AudioCue {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_common_lua_fields(fields);
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        add_common_lua_methods(methods)
    }
}
