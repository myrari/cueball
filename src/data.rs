use serde::{Serialize,Deserialize};
use std::cmp::max;

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


// Possibly change time representation later.
// For now this is a float of seconds.
pub type CueTime = f64;

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

    fn get_referents(&self)                -> Vec<&String> {Vec::new()}

    fn is_enabled(&self)             -> bool {false}
    fn set_enabled(&self, _to: bool) -> () {}
    fn is_armed(&self)               -> bool {false}
    fn set_armed(&self, _to: bool)   -> () {}
    fn is_errored(&self)             -> bool {false}
    fn can_fire(&self)               -> bool {
        self.is_enabled()
            && self.is_armed()
            && !self.is_errored()
    }

    fn is_networked(&self)          -> bool {false}

    fn go(&self)                    -> () {}
    fn running(&self)               -> CueRunning {CueRunning::Stopped}
    fn stop(&self)                  -> () {}
    fn set_paused(&self, _pu: bool)  -> () {}

    fn bounded(&self)   -> bool            {false}
    fn length(&self)    -> Option<CueTime> {None}
    fn elapsed(&self)   -> Option<CueTime> {None}
    fn remaining(&self) -> Option<CueTime> {None}
    fn reset(&mut self)     -> Result<(), ()>  {Err(())}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CueRunning {
    Running,
    Paused,
    Stopped
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
