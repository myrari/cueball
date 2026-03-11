use crate::{
    cues::{CueRunning, CueTime},
    CueList,
};

use super::{add_common_lua_fields, add_common_lua_methods, Cue};
use log::debug;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GroupType {
    Sync,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GroupCue {
    pub id: String,
    pub name: String,

    pub typ: GroupType,
    pub cues: CueList,
}

impl GroupCue {
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: "New remark cue".to_string(),
            typ: GroupType::Sync,
            cues: CueList::new(),
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
        debug!("Go Group {}", self.name);

        match self.typ {
            GroupType::Sync => {
                for c in &mut self.cues {
                    c.go();
                }
            }
        }
    }

    fn running(&self) -> CueRunning {
        self.cues
            .into_iter()
            .fold(CueRunning::Stopped, |acc, c| match (&acc, c.running()) {
                (CueRunning::Running, _) => CueRunning::Running,
                (_, CueRunning::Running) => CueRunning::Running,
                (CueRunning::Paused, _) => CueRunning::Paused,
                (_, CueRunning::Paused) => CueRunning::Paused,
                _ => acc,
            })
    }

    fn stop(&mut self) -> () {
        for c in &mut self.cues {
            c.stop();
        }
    }

    fn set_paused(&mut self, pu: bool) -> () {
        for c in &mut self.cues {
            c.set_paused(pu);
        }
    }

    fn length(&self) -> Option<CueTime> {
        match self.typ {
            GroupType::Sync => {
                self.cues
                    .into_iter()
                    .fold(None, |acc, c| match (&acc, c.length()) {
                        (_, None) => acc,
                        (None, Some(t)) => Some(t),
                        (Some(t1), Some(t2)) => Some(t2.max(*t1)),
                    })
            }
        }
    }

    fn elapsed(&self) -> Option<CueTime> {
        match self.typ {
            GroupType::Sync => {
                self.cues
                    .into_iter()
                    .fold(None, |acc, c| match (&acc, c.elapsed()) {
                        (_, None) => acc,
                        (None, Some(t)) => Some(t),
                        (Some(t1), Some(t2)) => Some(t2.max(*t1)),
                    })
            }
        }
    }

    fn remaining(&self) -> Option<CueTime> {
        if let Some(length) = self.length() {
            if let Some(elapsed) = self.elapsed() {
                return Some(length - elapsed);
            }
        }
        None
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
