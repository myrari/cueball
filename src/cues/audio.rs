use std::{fs::File, io::BufReader};

use crate::audio;
use anyhow::anyhow;
use log::error;
use mlua::prelude::*;
use rodio::{Decoder, Sink};
use serde::{Deserialize, Serialize};

use super::{add_common_lua_fields, add_common_lua_methods, Cue};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AudioCue {
    pub id: String,
    pub name: String,

    pub file_path: String,
}

impl AudioCue {
    pub fn with_id(id: String) -> Self {
        Self {
            id,
            name: "New audio cue".into(),
            file_path: "".into(),
        }
    }

    fn play_audio(&mut self) -> Result<(), anyhow::Error> {
        audio::AUDIO_MANAGER.with_borrow_mut(|am| {
            if let Some(am) = am {
                let file = BufReader::new(File::open(self.file_path.clone())?);

                let source = Decoder::new(file)?;

                let sink = Sink::try_new(&am.handle)?;
                sink.append(source);

                sink.detach();

                Ok(())
            } else {
                Err(anyhow!("Audio Manager not initialized!"))
            }
        })
    }
}

#[typetag::serde]
impl Cue for AudioCue {
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
