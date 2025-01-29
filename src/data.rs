use serde::{Serialize,Deserialize};
use std::cmp::max;

pub struct CueList {
    pub list: Vec<Box<dyn Cue>>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    fn get_new_cue_id(&self) -> u64 {
        let mut largest_id = 0;

        for cue in &self.list {
            largest_id = max(cue.id_num().unwrap_or(0), largest_id);
        }

        largest_id + 1
    }
}

pub trait Cue {
    fn id(&self)                    -> String;
    fn id_num(&self)                -> Option<u64> {
        self.id().parse::<u64>().ok()
    }
    fn type_str_full(&self)         -> String;
    fn type_str_short(&self)        -> String;
    fn name(&self)                  -> Option<String>;
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

// Possibly change time representation later.
// For now this is a float of seconds.
pub type CueTime = f64;

pub trait CueTimed: CueRunnable {
    fn bounded()   -> bool;
    fn length()    -> Option<CueTime>;
    fn elapsed()   -> Option<CueTime>;
    fn remaining() -> Option<CueTime>;
}
