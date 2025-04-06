use std::{fmt::Debug, fs::File, io::BufReader};

use crate::audio;
use anyhow::anyhow;
use log::{debug, error, info};
use mlua::prelude::*;
use rodio::{Decoder, Sink};
use serde::{Deserialize, Serialize};

use super::{add_common_lua_fields, add_common_lua_methods, Cue};

#[derive(Serialize, Deserialize)]
pub struct AudioCue {
    pub id: String,
    pub name: String,

    pub file_path: String,

    #[serde(skip)]
    pub sink: Option<Box<Sink>>,
}

impl AudioCue {
    pub fn with_id(id: String) -> Self {
        Self {
            id,
            name: "New audio cue".into(),
            file_path: "".into(),
            sink: None,
        }
    }

    fn play_audio(&mut self) -> Result<(), anyhow::Error> {
        if let Some(sink) = &self.sink {
            let file = BufReader::new(File::open(self.file_path.clone())?);

            let source = Decoder::new(file)?;

            sink.append(source);

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
                    sink: Some(Box::new(
                        Sink::try_new(&am.handle).expect("Failed to create sink for audio cue"),
                    )),
                }
            } else {
                Self {
                    id: self.id.clone(),
                    name: self.name.clone(),
                    file_path: self.file_path.clone(),
                    sink: None,
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
        info!("Init audio cue {}", self.id);
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
        })
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
}

impl LuaUserData for AudioCue {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_common_lua_fields(fields);
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        add_common_lua_methods(methods)
    }
}
