use serde::{Serialize,Deserialize};
use std::cmp::max;
use mlua::prelude::*;

pub struct CueList {
    pub list: Vec<Box<dyn Cue>>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, new_cue: Box<dyn Cue>) -> Result<(), ()> {
        if self.consistency_checks_add(&new_cue) {
            self.list.push(new_cue);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_new_cue_id(&self) -> u64 {
        let mut largest_id = 0;

        for cue in &self.list {
            largest_id = max(cue.get_id_num().unwrap_or(0), largest_id);
        }

        largest_id + 1
    }

    pub fn consistency_checks_add(&self, new_cue: &Box<dyn Cue>) -> bool {
        // FIXME: this should also check that all referents exist for
        // instances of CueReferencing.
        self.id_uniqueness_check(&new_cue.get_id())
    }
    fn id_uniqueness_check(&self, _new_id: &String) -> bool {true} // FIXME
}

pub trait Cue {
    fn get_id(&self)                       -> String;
    fn set_id(&mut self, new_id: &str)         -> ();
    fn get_id_num(&self)                   -> Option<u64> {
        self.get_id().parse::<u64>().ok()
    }
    fn get_name(&self)                     -> String;
    fn set_name(&mut self, new_name: &str) -> ();
    fn type_str_full(&self)                -> String;
    fn type_str_short(&self)               -> String;
}

pub trait CueReferencing: Cue {
    fn get_referents(&self)                -> Vec<&String>;
}

pub trait CueRunnable: Cue {
    fn is_enabled(&self)            -> bool;
    fn set_enabled(&self, to: bool) -> ();
    fn is_armed(&self)              -> bool;
    fn set_armed(&self, to: bool)   -> ();
    fn is_errored(&self)            -> bool; // Possibly convert to Option later
    fn can_fire(&self)              -> bool;

    fn is_networked(&self)          -> bool;

    fn go(&self)                    -> ();
    fn running(&self)               -> CueRunning;
    fn stop(&self)                  -> ();
    fn set_paused(&self, pu: bool)  -> ();
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CueRunning {
    Running,
    Paused,
    Stopped
}
impl IntoLua for CueRunning {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        Ok(format!("{:?}", self).into_lua(lua)?)
    }
}

// Possibly change time representation later.
// For now this is a float of seconds.
pub type CueTime = f64;

pub trait CueTimed: CueRunnable {
    fn bounded()   -> bool;
    fn length()    -> Option<CueTime>;
    fn elapsed()   -> Option<CueTime>;
    fn remaining() -> Option<CueTime>;
    fn reset()     -> Result<(), ()>;
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RemarkCue {
    pub id: String,
    pub name: String,
    pub notes: String
}
impl Default for RemarkCue {
    fn default() -> Self {
        RemarkCue {
            id: "0".to_string(),
            name: "New remark cue".to_string(),
            notes: "".to_string()
        }
    }
}
impl Cue for RemarkCue {
    fn get_id(&self)         -> String {self.id.clone()}
    fn get_name(&self)       -> String {self.name.clone()}
    fn set_id(&mut self, new_id: &str) -> () {
        self.id = new_id.to_string();
    }
    fn set_name(&mut self, new_name: &str) -> () {
        self.name = new_name.to_string();
    }
    fn type_str_full(&self)  -> String {"Remark".to_string()}
    fn type_str_short(&self) -> String {"Rmk".to_string()}
}

impl LuaUserData for Box<dyn Cue> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // This might get removed depending on if ID storage changes
        fields.add_field_method_get("id",   |_, this| Ok(this.get_id()));
        // Add method for setting ID
        fields.add_field_method_get("name", |_, this| Ok(this.get_name()));
        fields.add_field_method_set("name", |_, this, new_name: String|
            Ok(this.set_name(&new_name)));
        fields.add_field_method_get("type_s", |_, this|
            Ok(this.type_str_short()));
        fields.add_field_method_get("type", |_, this| Ok(this.type_str_full()));
    }
}

impl LuaUserData for Box<dyn CueRunnable> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("enabled",  |_, this|
            Ok(this.is_enabled()));
        fields.add_field_method_set("enabled",  |_, this, enabled: bool|
            Ok(this.set_enabled(enabled)));
        fields.add_field_method_get("armed",    |_, this|
            Ok(this.is_armed()));
        fields.add_field_method_set("armed",    |_, this, armed: bool|
            Ok(this.set_armed(armed)));
        fields.add_field_method_get("errored",  |_, this|
            Ok(this.is_errored()));
        fields.add_field_method_get("can_fire", |_, this| Ok(this.can_fire()));
        fields.add_field_method_get("running",  |_, this| Ok(this.running()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("go",         |_, this, ()| Ok(this.go()));
        methods.add_method_mut("stop",       |_, this, ()| Ok(this.stop()));
        methods.add_method_mut("set_paused",
            |_, this, x: bool| Ok(this.set_paused(x)));
    }
}
