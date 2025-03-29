use crate::{cue_imp::BonkCue, RemarkCue};

pub trait Inspector {
    // unique because ALL cues will show basics tab
    fn basics(&mut self, _ui: &mut egui::Ui) -> () {
        ()
    }

    fn time_and_loops(&mut self) -> Option<Box<dyn Fn(&mut egui::Ui)>> {
        None
    }
}

#[derive(Debug)]
pub struct RemarkCueInspector<'a> {
    pub cue: &'a mut RemarkCue,
}

impl Inspector for RemarkCueInspector<'_> {
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

impl Inspector for BonkCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("Bonk count: ");
            ui.label(self.cue.ctr.to_string());
        });
    }
}
