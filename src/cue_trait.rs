use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::max;

use crate::cue_disp::Inspector;

#[derive(Serialize, Deserialize)]
pub struct CueList {
    pub list: Vec<Box<dyn Cue>>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, new_cue: impl Cue + 'static) -> Result<usize, ()> {
        if self.consistency_checks_add(&new_cue) {
            self.list.push(Box::new(new_cue));
            Ok(self.list.len() - 1)
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

    pub fn get_cue(&self, id: String) -> Option<&Box<dyn Cue>> {
        for cue in &self.list {
            if cue.get_id() == id {
                return Some(cue);
            }
        }
        None
    }

    pub fn get_cue_mut(&mut self, id: String) -> Option<&mut Box<dyn Cue>> {
        for cue in &mut self.list {
            if cue.get_id() == id {
                return Some(cue);
            }
        }
        None
    }

    pub fn consistency_checks_add(&self, new_cue: &impl Cue) -> bool {
        // FIXME: this should also check that all referents exist for
        // instances of CueReferencing.
        self.id_uniqueness_check(&new_cue.get_id())
    }
    fn id_uniqueness_check(&self, _new_id: &String) -> bool {
        true
    } // FIXME
}

#[typetag::serde(tag = "type")]
pub trait Cue {
    fn get_id(&self) -> String;
    fn set_id(&mut self, new_id: &str) -> ();
    fn get_id_num(&self) -> Option<u64> {
        self.get_id().parse::<u64>().ok()
    }
    fn get_name(&self) -> String;
    fn set_name(&mut self, new_name: &str) -> ();
    fn type_str_full(&self) -> String;
    fn type_str_short(&self) -> String;
    fn get_attributes(&self) -> CueTypeAttributes {
        CueTypeAttributes::default()
    }

    fn get_referents(&self) -> Vec<&String> {
        Vec::new()
    }

    fn is_enabled(&self) -> bool {
        false
    }
    fn set_enabled(&mut self, _to: bool) -> () {}
    fn is_armed(&self) -> bool {
        false
    }
    fn set_armed(&mut self, _to: bool) -> () {}
    fn is_errored(&self) -> bool {
        false
    }
    fn can_fire(&self) -> bool {
        self.is_enabled() && self.is_armed() && !self.is_errored()
    }

    fn go(&mut self) -> ();
    fn running(&self) -> CueRunning {
        CueRunning::Stopped
    }
    fn stop(&mut self) -> () {}
    fn set_paused(&mut self, _pu: bool) -> () {}

    fn length(&self) -> Option<CueTime> {
        None
    }
    fn elapsed(&self) -> Option<CueTime> {
        None
    }
    fn remaining(&self) -> Option<CueTime> {
        None
    }
    fn reset(&mut self) -> Result<(), ()> {
        Err(())
    }

    // offset for playhead after playing this cue meant to be overridden by
    // group cues or other things that should advance by more than one cue at
    // a time
    fn next_offset(&self) -> usize {
        1
    }

    fn with_id(id: String) -> Self
    where
        Self: Sized;

    fn inspector(&mut self) -> Option<Box<dyn Inspector + '_>> {
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CueRunning {
    Running,
    Paused,
    Stopped,
}
impl IntoLua for CueRunning {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        Ok(format!("{:?}", self).into_lua(lua)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CueTypeAttributes {
    pub runnable: bool,
    pub timed: bool,
    pub timed_bounded: bool,
    pub networked: Option<bool>,
    pub idempotent: bool,
    pub tc: bool,
}
impl Default for CueTypeAttributes {
    fn default() -> Self {
        CueTypeAttributes {
            runnable: false,
            timed: false,
            timed_bounded: false,
            networked: Some(false),
            idempotent: true,
            tc: false,
        }
    }
}
// Possibly change time representation later.
// For now this is a float of seconds.
pub type CueTime = f64;

impl LuaUserData for Box<dyn Cue> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // This might get removed depending on if ID storage changes
        fields.add_field_method_get("id", |_, this| Ok(this.get_id()));
        // Add method for setting ID
        fields.add_field_method_get("name", |_, this| Ok(this.get_name()));
        fields.add_field_method_set("name", |_, this, new_name: String| {
            Ok(this.set_name(&new_name))
        });
        fields.add_field_method_get("type_s", |_, this| Ok(this.type_str_short()));
        fields.add_field_method_get("type", |_, this| Ok(this.type_str_full()));
        fields.add_field_method_get("enabled", |_, this| Ok(this.is_enabled()));
        fields.add_field_method_set("enabled", |_, this, enabled: bool| {
            Ok(this.set_enabled(enabled))
        });
        fields.add_field_method_get("armed", |_, this| Ok(this.is_armed()));
        fields.add_field_method_set("armed", |_, this, armed: bool| Ok(this.set_armed(armed)));
        fields.add_field_method_get("errored", |_, this| Ok(this.is_errored()));
        fields.add_field_method_get("can_fire", |_, this| Ok(this.can_fire()));
        fields.add_field_method_get("running", |_, this| Ok(this.running()));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("go", |_, this, ()| Ok(this.go()));
        methods.add_method_mut("stop", |_, this, ()| Ok(this.stop()));
        methods.add_method_mut("set_paused", |_, this, x: bool| Ok(this.set_paused(x)));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CLIMode {
    CLI,
    Lua,
}
impl std::fmt::Display for CLIMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
