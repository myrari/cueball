use log::{debug, info};
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    add_common_lua_fields, add_common_lua_methods, Cue, CueTypeAttributes,
};

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
        debug!("Audio {}", self.name)
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RemarkCue {
    pub id: String,
    pub name: String,
    pub notes: String,
}

impl RemarkCue {
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: "New remark cue".to_string(),
            notes: "".to_string(),
        }
    }
}

#[typetag::serde]
impl Cue for RemarkCue {
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
        "Remark".to_string()
    }
    fn type_str_short(&self) -> String {
        "Rmk".to_string()
    }
    fn go(&mut self) -> () {
        debug!("Remark {}", self.name)
    }
}

impl LuaUserData for RemarkCue {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_common_lua_fields(fields);
        fields.add_field_method_get("notes", |_, this| Ok(this.notes.clone()));
        fields.add_field_method_set("notes", |_, this, new_notes: String| {
            Ok(this.notes = new_notes)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        add_common_lua_methods(methods)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BonkCue {
    id: String,
    name: String,
    enabled: bool,
    armed: bool,
    pub ctr: u64,
}

impl BonkCue {
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: "New bonk cue".to_string(),
            ctr: 0,
            enabled: true,
            armed: true,
        }
    }
}

#[typetag::serde]
impl Cue for BonkCue {
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
        "Bonk".to_string()
    }
    fn type_str_short(&self) -> String {
        "Bonk".to_string()
    }
    fn get_attributes(&self) -> CueTypeAttributes {
        let mut a = CueTypeAttributes::default();
        a.runnable = true;
        a
    }

    fn get_referents(&self) -> Vec<&String> {
        Vec::new()
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, to: bool) -> () {
        self.enabled = to;
    }
    fn is_armed(&self) -> bool {
        self.armed
    }
    fn set_armed(&mut self, to: bool) -> () {
        self.armed = to;
    }
    fn is_errored(&self) -> bool {
        false
    }

    fn go(&mut self) -> () {
        if self.can_fire() {
            info!("bonk #{}", self.ctr);
            self.ctr += 1;
        }
    }
    fn reset(&mut self) -> Result<(), ()> {
        self.ctr = 0;
        Ok(())
    }
}

impl LuaUserData for BonkCue {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_common_lua_fields(fields);
        fields.add_field_method_get("ctr", |_, this| Ok(this.ctr));
        fields.add_field_method_set("ctr", |_, this, new_ctr: u64| Ok(this.ctr = new_ctr));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        add_common_lua_methods(methods)
    }
}
