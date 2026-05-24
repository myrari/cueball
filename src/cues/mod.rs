mod audio;
mod cues;
mod group;

pub use audio::AudioCue;
pub use cues::{BonkCue, RemarkCue};
pub use group::GroupCue;

use log::warn;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cmp::max, path::PathBuf};

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
            MultitypeCue::Audio(c)   => c.$method($($x,)*),
            MultitypeCue::Group(c)   => c.$method($($x,)*),
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Cue {
    fn init(&mut self) -> ();

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
pub enum MultitypeCue {
    // modify this when adding/deleting cues
    Remark(RemarkCue),
    Bonk(BonkCue),
    Audio(AudioCue),
    Group(GroupCue),
}

#[typetag::serde]
impl Cue for MultitypeCue {
    // modify this when adding/deleting/modifying Cue methods
    call_cue_enum_inner!(
        fn init(&mut self) -> ();
    );
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: Option<PathBuf>,

    pub cues: CueList,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("Untitled"),
            path: None,
            cues: CueList::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CueList {
    list: Vec<MultitypeCue>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn init_cues(&mut self) -> () {
        for cue in &mut self.list {
            cue.init();
        }
    }

    pub fn go(&mut self, i: usize) -> usize {
        let len = self.len();
        let depth = self.get_cue_depth(i);
        let cue = &mut self[i];
        match cue {
            MultitypeCue::Group(gc) => match gc.typ {
                group::GroupType::Sync => {
                    let offset = gc.next_offset();
                    let remaining_cues = len - i - 1;
                    if gc.len > remaining_cues {
                        // clamp group length to remaining cues in cue list
                        gc.len = remaining_cues;
                    }

                    for j in 0..gc.len {
                        // ignore offset values for nested cues
                        // BUT skip over cues in a nested group
                        if self.get_cue_depth(i + j + 1) == depth + 1 {
                            let _ = self.go(i + j + 1);
                        }
                    }

                    offset
                }
            },
            c => {
                let offset = c.next_offset();
                c.go();
                offset
            }
        }
    }

    pub fn add(&mut self, cue: MultitypeCue) -> Result<usize, ()> {
        if self.consistency_checks_add(&cue) {
            let mut new_cue = cue;
            new_cue.init();
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

    pub fn get_cue_depth(&self, i: usize) -> usize {
        let mut group_lens: Vec<usize> = vec![];

        for j in 0..i {
            match &self[j] {
                MultitypeCue::Group(gc) => {
                    group_lens.push(gc.len);
                }
                _ => {}
            }
            group_lens = group_lens
                .iter()
                .filter_map(|l| if *l < 1 { None } else { Some(l - 1) })
                .collect();
        }

        group_lens.len()
    }

    pub fn get_cue_parent(&self, i: usize) -> Option<usize> {
        let mut groups: Vec<(usize, usize)> = vec![];

        for j in 0..i {
            match &self[j] {
                MultitypeCue::Group(gc) => {
                    groups.push((j, gc.len));
                }
                _ => {}
            }
            groups = groups
                .iter()
                .filter_map(|(k, l)| if *l < 1 { None } else { Some((*k, l - 1)) })
                .collect();
        }

        Some(groups.pop()?.0)
    }

    pub fn get_all_cue_parents(&self, i: usize) -> Vec<usize> {
        let mut out: Vec<usize> = vec![];

        let mut cur = i;

        while let Some(p) = self.get_cue_parent(cur) {
            out.push(p);
            cur = p;
        }

        out
    }

    pub fn move_cue(&mut self, mve: usize, to: usize) -> () {
        // move "mve" cue to "to" cue

        for mve_parent in self.get_all_cue_parents(mve) {
            match &mut self[mve_parent] {
                MultitypeCue::Group(gc) => gc.len -= 1,
                _ => warn!("Cue {mve_parent} is a parent but not a group cue!"),
            }
        }

        for to_parent in self.get_all_cue_parents(to) {
            match &mut self[to_parent] {
                MultitypeCue::Group(gc) => gc.len += 1,
                _ => warn!("Cue {to_parent} is a parent but not a group cue!"),
            }
        }

        if mve < to {
            // moving down the list
            let len = to - mve;
            // extra step for moving down into group cues
            match &mut self[to] {
                MultitypeCue::Group(gc) => gc.len += 1,
                _ => {}
            }

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

impl<'a> IntoIterator for &'a CueList {
    type Item = &'a MultitypeCue;

    type IntoIter = std::slice::Iter<'a, MultitypeCue>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

impl<'a> IntoIterator for &'a mut CueList {
    type Item = &'a mut MultitypeCue;

    type IntoIter = std::slice::IterMut<'a, MultitypeCue>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter_mut()
    }
}

impl std::ops::Index<usize> for CueList {
    type Output = MultitypeCue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

impl std::ops::IndexMut<usize> for CueList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index]
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
pub type CueTime = f32;

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
