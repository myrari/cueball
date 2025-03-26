use crate::RemarkCue;

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
        ui.label("remark cue!!!");
        ui.label(self.cue.notes.clone());
    }

    fn time_and_loops(&mut self) -> Option<Box<dyn Fn(&mut egui::Ui)>> {
        Some(Box::new(|ui: &mut egui::Ui| {
            ui.label("time and loops!!");
        }))
    }
}
