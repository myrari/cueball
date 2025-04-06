use crate::{
    cues::{AudioCue, BonkCue, RemarkCue},
    MultitypeCue,
};

pub trait CueInspector {
    // unique because ALL cues will show basics tab
    fn basics(&mut self, _ui: &mut egui::Ui) -> () {
        ()
    }

    fn time_and_loops(&mut self) -> Option<Box<dyn Fn(&mut egui::Ui)>> {
        None
    }
}

pub fn get_cue_inspector(cue: &mut MultitypeCue) -> Option<Box<dyn CueInspector + '_>> {
    match cue {
        MultitypeCue::Remark(ref mut q) => Some(Box::new(RemarkCueInspector { cue: q })),
        MultitypeCue::Bonk(ref mut q) => Some(Box::new(BonkCueInspector { cue: q })),
        MultitypeCue::Audio(ref mut q) => Some(Box::new(AudioCueInspector { cue: q })),
    }
}

#[derive(Debug)]
struct AudioCueInspector<'a> {
    pub cue: &'a mut AudioCue,
}

impl CueInspector for AudioCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("File: ");
            ui.text_edit_singleline(&mut self.cue.file_path);
        });
    }
}

#[derive(Debug)]
pub struct RemarkCueInspector<'a> {
    pub cue: &'a mut RemarkCue,
}

impl CueInspector for RemarkCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("Notes: ");
            ui.text_edit_singleline(&mut self.cue.notes);
        });
    }

    fn time_and_loops(&mut self) -> Option<Box<dyn Fn(&mut egui::Ui)>> {
        Some(Box::new(|ui: &mut egui::Ui| {
            ui.label("time and loops!!");
        }))
    }
}

#[derive(Debug)]
pub struct BonkCueInspector<'a> {
    pub cue: &'a mut BonkCue,
}

impl CueInspector for BonkCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("Bonk count: ");
            ui.label(self.cue.ctr.to_string());
        });
    }
}
