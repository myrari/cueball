use crate::{
    cues::{BonkCue, RemarkCue},
    MultitypeCue,
};

mod audio;

pub use audio::AudioCueInspector;

pub trait CueInspector {
    // unique because ALL cues will show basics tab
    fn basics(&mut self, _ui: &mut egui::Ui) -> () {
        ()
    }

    fn time_and_loops(&self) -> bool {
        false
    }
    fn time_and_loops_fn(&mut self, _ui: &mut egui::Ui) -> () {}
}

pub fn get_cue_inspector(cue: &mut MultitypeCue) -> Option<Box<dyn CueInspector + '_>> {
    match cue {
        MultitypeCue::Remark(ref mut q) => Some(Box::new(RemarkCueInspector::new(q))),
        MultitypeCue::Bonk(ref mut q) => Some(Box::new(BonkCueInspector::new(q))),
        MultitypeCue::Audio(ref mut q) => Some(Box::new(AudioCueInspector::new(q))),
    }
}

#[derive(Debug)]
pub struct RemarkCueInspector<'a> {
    pub cue: &'a mut RemarkCue,
}

impl<'a> RemarkCueInspector<'a> {
    fn new(cue: &'a mut RemarkCue) -> Self {
        Self {
            cue
        }
    }
}

impl CueInspector for RemarkCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("Notes: ");
            ui.text_edit_singleline(&mut self.cue.notes);
        });
    }
}

#[derive(Debug)]
pub struct BonkCueInspector<'a> {
    pub cue: &'a mut BonkCue,
}

impl<'a> BonkCueInspector<'a> {
    fn new(cue: &'a mut BonkCue) -> Self {
        Self {
            cue
        }
    }
}

impl CueInspector for BonkCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("Bonk count: ");
            ui.label(self.cue.ctr.to_string());
        });
    }
}
