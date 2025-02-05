use serde::{Serialize,Deserialize};
use crate::Cue;

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
