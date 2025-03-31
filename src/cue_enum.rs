use crate::{Cue, RemarkCue, BonkCue, CueTime, CueRunning, CueTypeAttributes,
    Inspector};
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

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
            MultitypeCue::Remark(c) => c.$method($($x,)*),
            MultitypeCue::Bonk(c)   => c.$method($($x,)*),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MultitypeCue {
    Remark(RemarkCue),
    Bonk(BonkCue),
}

#[typetag::serde]
impl Cue for MultitypeCue {
    call_cue_enum_inner!(fn get_id(&self) -> String;);
    call_cue_enum_inner!(fn set_id(&mut self, new_id: &str) -> (););
    call_cue_enum_inner!(fn get_id_num(&self) -> Option<u64>;);
    call_cue_enum_inner!(fn get_name(&self) -> String;);
    call_cue_enum_inner!(fn set_name(&mut self, new_name: &str) -> (););
    call_cue_enum_inner!(fn type_str_full(&self) -> String;);
    call_cue_enum_inner!(fn type_str_short(&self) -> String;);
    call_cue_enum_inner!(fn get_attributes(&self) -> CueTypeAttributes;);
    call_cue_enum_inner!(fn get_referents(&self) -> Vec<&String>;);
    call_cue_enum_inner!(fn is_enabled(&self) -> bool;);
    call_cue_enum_inner!(fn set_enabled(&mut self, _to: bool) -> (););
    call_cue_enum_inner!(fn is_armed(&self) -> bool;);
    call_cue_enum_inner!(fn set_armed(&mut self, _to: bool) -> (););
    call_cue_enum_inner!(fn is_errored(&self) -> bool;);
    call_cue_enum_inner!(fn can_fire(&self) -> bool;);
    call_cue_enum_inner!(fn go(&mut self) -> (););
    call_cue_enum_inner!(fn running(&self) -> CueRunning;);
    call_cue_enum_inner!(fn stop(&mut self) -> (););
    call_cue_enum_inner!(fn set_paused(&mut self, _pu: bool) -> (););
    call_cue_enum_inner!(fn length(&self) -> Option<CueTime>;);
    call_cue_enum_inner!(fn elapsed(&self) -> Option<CueTime>;);
    call_cue_enum_inner!(fn remaining(&self) -> Option<CueTime>;);
    call_cue_enum_inner!(fn reset(&mut self) -> Result<(), ()>;);
    call_cue_enum_inner!(fn next_offset(&self) -> usize;);
    call_cue_enum_inner!(fn inspector(&mut self) -> Option<Box<dyn Inspector + '_>>;);
}

impl IntoLua for MultitypeCue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        call_cue_enum_inner_matchblock!(self, into_lua, lua)
    }
}
