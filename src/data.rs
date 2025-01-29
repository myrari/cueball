#[derive(Debug)]
pub struct CueList {
    pub list: Vec<Cue>,
}

impl CueList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, cue_type: CueType) {
        let cue = Cue {
            id: self.get_new_cue_id(),
            name: match &cue_type {
                CueType::Message(msg) => msg.clone(),
                CueType::Process => String::from("Process cue"),
            },
            cue_type,
        };
        self.list.push(cue);
    }

    fn get_new_cue_id(&self) -> u64 {
        let mut largest_id = 0;

        for cue in &self.list {
            if cue.id > largest_id {
                largest_id = cue.id;
            }
        }

        largest_id + 1
    }
}

#[derive(Debug)]
pub struct Cue {
    pub id: u64,
    pub name: String,

    pub cue_type: CueType,
}

#[derive(Debug)]
pub enum CueType {
    Message(String),
    Process,
}
