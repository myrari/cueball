mod cues;

pub use cues::{BonkCue, RemarkCue};

use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::max;

/*******************************************************************************
* 0. NOTE TO FUTURE MAINTAINERS:                                               *
* You do not have to understand how any of this works. All of this complexity  *
* exists precisely so that Cueball can be maintained without having to deal    *
* with either tedium or cursedness. However, there are two things that will    *
* become necessary in the future, to wit:                                      *
*   1. Adding new cues or deleting existing ones.                              *
*   2. Adding, modifying, or deleting methods of the `Cue` trait.              *
* Both of these will require modifications to parts of this file. Such parts   *
* are signposted with comments, and the modifications required are described   *
* herefrom.                                                                    *
*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=
* 1. Adding or deleting cues                                                   *
* To add or delete a cue, be sure to make a corresponding modification to the  *
* MultitypeCue enum and the call_cue_enum_inner_matchblock macro at the marked *
* points. The modifications required should be trivial copy-paste additions    *
* when adding a cue, and trivial deletions when deleting a cue.                *
*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=
* 2. Modifying the `Cue` trait                                                 *
* If you modify the methods of the `Cue` trait, you must add/modify/delete the *
* corresponding lines in the trait implementation. Simply wrap the proper      *
* function signature in a call to call_cue_enum_inner, like so:                *
*   // cue_trait.rs                                                            *
*   trait Cue {                                                                *
*       ...                                                                    *
*       fn bonneville(&mut self, arg1: bool) -> RetType;                       *
*       ...                                                                    *
*   }                                                                          *
*                                                                              *
*   // cue_enum.rs                                                             *
*   impl Cue for MultitypeCue {                                                *
*       ...                                                                    *
*       call_cue_enum_inner!(fn bonneville(&mut self, arg1: bool) -> RetType); *
*       ...                                                                    *
*   }                                                                          *
* In most situations copy-pasting from one definition to the other should      *
* suffice.                                                                     *
*******************************************************************************/

macro_rules! call_cue_enum_inner {
    (fn $method:ident(&self $(,$x:ident: $t:ty),*) -> $ret:ty $(;)? $($_:block)?) => {
        fn $method(&self, $($x: $t)*) -> $ret {
            call_cue_enum_inner_matchblock!(self, $method, $($x)*)
        }
    };
    (fn $method:ident(&mut self $(,$x:ident: $t:ty),*) -> $ret:ty $(;)? $($_:block)?) => {
        fn $method(&mut self, $($x: $t)*) -> $ret {
            call_cue_enum_inner_matchblock!(self, $method, $($x)*)
        }
    };
}

macro_rules! call_cue_enum_inner_matchblock {
    ($self:ident, $method:ident, $($x:ident),*) => {
        match $self {
            // modify this when adding/deleting cues
            MultitypeCue::Remark(c) => c.$method($($x,)*),
            MultitypeCue::Bonk(c)   => c.$method($($x,)*),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MultitypeCue {
    // modify this when adding/deleting cues
    Remark(RemarkCue),
    Bonk(BonkCue),
}

#[typetag::serde]
impl Cue for MultitypeCue {
    // modify this when adding/deleting/modifying Cue methods
    call_cue_enum_inner!(
        fn get_id(&self) -> String;
    );
    call_cue_enum_inner!(
        fn set_id(&mut self, new_id: &str) -> ();
    );
    call_cue_enum_inner!(
        fn get_id_num(&self) -> Option<u64>;
    );
    call_cue_enum_inner!(
        fn get_name(&self) -> String;
    );
    call_cue_enum_inner!(
        fn set_name(&mut self, new_name: &str) -> ();
    );
    call_cue_enum_inner!(
        fn type_str_full(&self) -> String;
    );
    call_cue_enum_inner!(
        fn type_str_short(&self) -> String;
    );
    call_cue_enum_inner!(
        fn get_attributes(&self) -> CueTypeAttributes;
    );
    call_cue_enum_inner!(
        fn get_referents(&self) -> Vec<&String>;
    );
    call_cue_enum_inner!(
        fn is_enabled(&self) -> bool;
    );
    call_cue_enum_inner!(
        fn set_enabled(&mut self, _to: bool) -> ();
    );
    call_cue_enum_inner!(
        fn is_armed(&self) -> bool;
    );
    call_cue_enum_inner!(
        fn set_armed(&mut self, _to: bool) -> ();
    );
    call_cue_enum_inner!(
        fn is_errored(&self) -> bool;
    );
    call_cue_enum_inner!(
        fn can_fire(&self) -> bool;
    );
    call_cue_enum_inner!(
        fn go(&mut self) -> ();
    );
    call_cue_enum_inner!(
        fn running(&self) -> CueRunning;
    );
    call_cue_enum_inner!(
        fn stop(&mut self) -> ();
    );
    call_cue_enum_inner!(
        fn set_paused(&mut self, _pu: bool) -> ();
    );
    call_cue_enum_inner!(
        fn length(&self) -> Option<CueTime>;
    );
    call_cue_enum_inner!(
        fn elapsed(&self) -> Option<CueTime>;
    );
    call_cue_enum_inner!(
        fn remaining(&self) -> Option<CueTime>;
    );
    call_cue_enum_inner!(
        fn reset(&mut self) -> Result<(), ()>;
    );
    call_cue_enum_inner!(
        fn next_offset(&self) -> usize;
    );
}

impl IntoLua for MultitypeCue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        call_cue_enum_inner_matchblock!(self, into_lua, lua)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CueList {
    pub list: Vec<MultitypeCue>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, new_cue: MultitypeCue) -> Result<usize, ()> {
        if self.consistency_checks_add(&new_cue) {
            self.list.push(new_cue);
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

    pub fn get_cue(&self, id: String) -> Option<&MultitypeCue> {
        for cue in &self.list {
            if cue.get_id() == id {
                return Some(cue);
            }
        }
        None
    }

    pub fn get_cue_mut(&mut self, id: String) -> Option<&mut MultitypeCue> {
        for cue in &mut self.list {
            if cue.get_id() == id {
                return Some(cue);
            }
        }
        None
    }

    pub fn move_cue(&mut self, mve: usize, to: usize) -> () {
        // move "mve" cue to "to" cue
        if mve < to {
            let len = to - mve;
            // moving down the list
            let slice = &mut self.list[mve..to + 1];
            for i in 0..len {
                let swap_to = len - i;
                slice.swap(0, swap_to);
            }
        } else if mve > to {
            // moving up the list
            let len = mve - to;
            let slice = &mut self.list[to..mve + 1];
            for i in 0..len {
                slice.swap(i, len);
            }
        } else {
            // moving cue to itself! do nothing
        }
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

pub fn add_common_lua_fields<Q: Cue, F: LuaUserDataFields<Q>>(fields: &mut F) {
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
pub fn add_common_lua_methods<Q: Cue, M: LuaUserDataMethods<Q>>(methods: &mut M) {
    methods.add_method_mut("go", |_, this, ()| Ok(this.go()));
    methods.add_method_mut("stop", |_, this, ()| Ok(this.stop()));
    methods.add_method_mut("set_paused", |_, this, x: bool| Ok(this.set_paused(x)));
}
