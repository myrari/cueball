use crate::{
    cues::{BonkCue, RemarkCue},
    MultitypeCue,
};

mod audio;

pub use audio::AudioCueInspector;

#[derive(Debug, PartialEq)]
pub enum InspectorPanelTabs {
    Basics,
    TimeLoops,
    Extra,
}

impl InspectorPanelTabs {
    pub const ITER: [(InspectorPanelTabs, &str); 3] = [
        (InspectorPanelTabs::Basics, "Basics"),
        (InspectorPanelTabs::TimeLoops, "Time & Loops"),
        (InspectorPanelTabs::Extra, "Extra"),
    ];
}

pub trait CueInspector {
    fn has_tab(&self, tab: &InspectorPanelTabs) -> bool {
        match tab {
            InspectorPanelTabs::Basics => true,
            _ => false,
        }
    }

    fn draw_tab(&mut self, _ui: &mut egui::Ui, _tab: &InspectorPanelTabs) {}
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
        Self { cue }
    }
}

impl CueInspector for RemarkCueInspector<'_> {
    fn draw_tab(&mut self, ui: &mut egui::Ui, tab: &InspectorPanelTabs) {
        match tab {
            InspectorPanelTabs::Basics => {
                ui.horizontal(|ui| {
                    ui.label("Notes: ");
                    ui.text_edit_singleline(&mut self.cue.notes);
                });
            }
            _ => {}
        };
    }
}

#[derive(Debug)]
pub struct BonkCueInspector<'a> {
    pub cue: &'a mut BonkCue,
}

impl<'a> BonkCueInspector<'a> {
    fn new(cue: &'a mut BonkCue) -> Self {
        Self { cue }
    }
}

impl CueInspector for BonkCueInspector<'_> {
    fn draw_tab(&mut self, ui: &mut egui::Ui, tab: &InspectorPanelTabs) {
        match tab {
            InspectorPanelTabs::Basics => {
                ui.horizontal(|ui| {
                    ui.label("Bonk count: ");
                    ui.label(self.cue.ctr.to_string());
                });
            }
            _ => {}
        };
    }
}
