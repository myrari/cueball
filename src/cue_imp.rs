use crate::{cue_disp::RemarkCueInspector, Cue, CueTypeAttributes, Inspector};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RemarkCue {
    pub id: String,
    pub name: String,
    pub notes: String,
}
impl Default for RemarkCue {
    fn default() -> Self {
        RemarkCue {
            id: "0".to_string(),
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

    fn with_id(id: String) -> Self
    where
        Self: Sized,
    {
        Self {
            id,
            ..Default::default()
        }
    }

    fn inspector(&mut self) -> Option<Box<dyn Inspector + '_>> {
        Some(Box::new(RemarkCueInspector {
            cue: self,
        }))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BonkCue {
    id: String,
    name: String,
    enabled: bool,
    armed: bool,
    ctr: u64,
}
impl Default for BonkCue {
    fn default() -> Self {
        BonkCue {
            id: "0".to_string(),
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

    fn with_id(id: String) -> Self
    where
        Self: Sized,
    {
        Self {
            id,
            ..Default::default()
        }
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
