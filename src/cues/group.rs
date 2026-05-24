use core::fmt;

use super::{add_common_lua_fields, add_common_lua_methods, Cue};
use log::warn;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GroupType {
    Sync,
}

impl fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GroupCue {
    pub id: String,
    pub name: String,

    pub typ: GroupType,
    pub len: usize,
}

impl GroupCue {
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: "New group cue".to_string(),
            typ: GroupType::Sync,
            len: 0,
        }
    }

    pub fn dry_clone(&self) -> Self {
        GroupCue {
            id: self.id.clone(),
            name: self.name.clone(),
            typ: self.typ.clone(),
            len: self.len,
        }
    }
}

#[typetag::serde]
impl Cue for GroupCue {
    fn init(&mut self) -> () {}

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
        "Group".to_string()
    }
    fn type_str_short(&self) -> String {
        "Grp".to_string()
    }

    fn go(&mut self) -> () {
        // a group cue doesn't do anything by itself on Go
        warn!(
            "Group {} was fired, group cues shouldn't be fired directly",
            self.name
        );
    }

    fn next_offset(&self) -> usize {
        match self.typ {
            GroupType::Sync => self.len + 1,
        }
    }
}

impl LuaUserData for GroupCue {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_common_lua_fields(fields);
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        add_common_lua_methods(methods)
    }
}
