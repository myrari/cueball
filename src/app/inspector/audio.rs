use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::anyhow;
use egui::{Color32, Id, Pos2, Rect, Sense, Stroke};
use log::error;
// use log::error;
use rodio::{Decoder, Source};

use crate::cues::AudioCue;

use super::CueInspector;

#[derive(Debug)]
pub struct AudioCueInspector<'a> {
    pub cue: &'a mut AudioCue,
}

impl<'a> AudioCueInspector<'a> {
    pub fn new(cue: &'a mut AudioCue) -> Self {
        Self { cue }
    }

    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("File: ");
            ui.text_edit_singleline(&mut self.cue.file_path);
        });
    }

    fn time_and_loops(&mut self, ui: &mut egui::Ui) -> () {
        let audio_data = match decode_source(&self.cue, ui) {
            Err(err) => {
                error!("Could not decode audio for cue {}: {}", self.cue.id, err);
                return;
            }
            Ok(d) => d,
        };

        // ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let total_width = ui.available_width();

            // left column
            ui.vertical(|ui| {
                ui.set_width(total_width * 0.3);

                // position in sink
                // ui.horizontal(|ui| {
                //     ui.label("Position: ");
                //     if let Some(sink) = &self.cue.sink {
                //         if sink.empty() {
                //             ui.label("Not playing");
                //         } else {
                //             ui.label(sink.get_pos().as_secs_f64().to_string());
                //             // ui.ctx().request_repaint();
                //         }
                //     } else {
                //         ui.label("Cue not initialized!");
                //     }
                // });

                // start and end offsets
                // ui.horizontal(|ui| {
                // let resp = ui.add(
                //     egui::TextEdit::singleline(&mut state.inspector_panel.id_buf)
                //         .font(TextStyle::Monospace)
                //         .desired_width(CUE_ID_WIDTH_PX),
                // );
                // if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                //     cue.set_id(state.inspector_panel.id_buf.as_str());
                // }
                // ui.add(egui::Slider::new(&mut self.cue.start, 0.0..=1.0).text("Start"));
                // ui.add(egui::Slider::new(&mut self.cue.end, 0.0..=1.0).text("End"));

                // ui.label("End: ");
                // let mut end_tmp = self.cue.end.to_string();
                // ui.text_edit_singleline(&mut end_tmp);
                // if let Ok(new) = end_tmp.parse() {
                //     self.cue.end = new;
                // }
                // });
            });

            // right column
            ui.vertical(|ui| {
                // sample views
                draw_waveform_view(self.cue, &audio_data, ui, total_width)
                // .unwrap_or_else(|err| error!("Error drawing audio waveform: {}", err))
            });
        });
    }
}

impl CueInspector for AudioCueInspector<'_> {
    fn has_tab(&self, tab: &super::InspectorPanelTabs) -> bool {
        match tab {
            super::InspectorPanelTabs::Basics => true,
            super::InspectorPanelTabs::TimeLoops => true,
            _ => false,
        }
    }

    fn draw_tab(&mut self, ui: &mut egui::Ui, tab: &super::InspectorPanelTabs) {
        match tab {
            super::InspectorPanelTabs::Basics => self.basics(ui),
            super::InspectorPanelTabs::TimeLoops => self.time_and_loops(ui),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct AudioData {
    samples: Arc<Mutex<Vec<i16>>>,
    rate: u32,
    duration: Option<Duration>,
}

fn draw_waveform_view(
    cue: &mut AudioCue,
    audio_data: &AudioData,
    ui: &mut egui::Ui,
    total_width: f32,
) -> Result<(), anyhow::Error> {
    let samples = match audio_data.samples.lock() {
        Err(err) => return Err(anyhow!(err.to_string())),
        Ok(ok) => ok,
    };

    let sink = match &cue.sink {
        None => return Err(anyhow!("Cue had no sync!")),
        Some(s) => s,
    };

    // if audio has known duration, use that
    // if not, try to calculate from number of samples
    let sample_len = match audio_data.duration {
        Some(d) => d.as_secs_f32(),
        None => (samples.len() as f32) / (2. * audio_data.rate as f32),
    };

    // let len = samples.len();

    // let visible_length_samples = (sample_rate as usize) * 1;
    // let start_idx = samples_played.saturating_sub(visible_length_samples / 2);
    // let start_idx = 0;
    // let end_idx = (start_idx + visible_length_samples).min(len);

    // let displayed_waveform = if start_idx < end_idx && len > 0 {
    //     &samples[start_idx..end_idx]
    // } else {
    //     &[] as &[i16]
    // };

    // only take every nth point
    const N: usize = 128;
    let displayed_waveform: Vec<&i16> = samples.iter().step_by(N).collect();

    let mut waveform_rect = ui.available_rect_before_wrap();
    waveform_rect.set_width(total_width * 0.7);
    let painter = ui.painter_at(waveform_rect);
    painter.rect_filled(waveform_rect, 4., Color32::BLACK);

    let horiz_scale = waveform_rect.width() / sample_len;

    if !displayed_waveform.is_empty() {
        let wave_height = waveform_rect.height() / (2. * 100000.);
        let wave_width = waveform_rect.width() / displayed_waveform.len().max(1) as f32;
        let center_y = waveform_rect.center().y;

        let points: Vec<Pos2> = displayed_waveform
            .iter()
            .enumerate()
            .map(|(i, sample)| {
                let x = waveform_rect.left_top().x + (i as f32 * wave_width);
                let y = center_y - ((**sample as f32) * wave_height);
                Pos2 { x, y }
            })
            .collect();

        painter.add(egui::Shape::line(
            points,
            Stroke::new(1.5, Color32::LIGHT_BLUE),
        ));

        if !sink.empty() {
            let time = sink.get_pos().as_secs_f32() + cue.start;
            let playhead_pos = time * horiz_scale;
            painter.add(egui::Shape::line_segment(
                [
                    Pos2 {
                        x: waveform_rect.left_top().x + playhead_pos,
                        y: waveform_rect.left_top().y,
                    },
                    Pos2 {
                        x: waveform_rect.left_top().x + playhead_pos,
                        y: waveform_rect.left_bottom().y,
                    },
                ],
                Stroke::new(1.5, Color32::YELLOW),
            ));

            ui.ctx().request_repaint();
        }
    }

    // start and end cutoffs
    let top = waveform_rect.left_top().y;
    let bottom = waveform_rect.left_bottom().y;

    let start_pos = cue.start * horiz_scale + waveform_rect.left_top().x;

    let start_icon_size = 16.;
    let start_icon_rect = Rect {
        min: Pos2 {
            x: start_pos - start_icon_size / 2.,
            y: top - start_icon_size,
        },
        max: Pos2 {
            x: start_pos + start_icon_size / 2.,
            y: top + start_icon_size,
        },
    };

    let start_icon_resp = ui.put(
        start_icon_rect,
        egui::Image::new(egui::include_image!("../../../assets/left-and-right.png"))
            .sense(Sense::drag()),
    );

    if start_icon_resp.dragged() {
        let delta = start_icon_resp.drag_delta().x / horiz_scale;
        let new_pos = (cue.start + delta).clamp(0., sample_len - cue.end);
        // cue.start = new_pos;
        let _ = cue.set_start(new_pos);
    }

    painter.add(egui::Shape::line_segment(
        [
            Pos2 {
                x: start_pos,
                y: top,
            },
            Pos2 {
                x: start_pos,
                y: bottom,
            },
        ],
        Stroke::new(1.5, Color32::BLUE),
    ));
    painter.rect_filled(
        egui::Rect {
            min: waveform_rect.left_top(),
            max: Pos2 {
                x: start_pos,
                y: bottom,
            },
        },
        0.,
        Color32::from_rgba_unmultiplied(0, 0, 200, 32),
    );

    let end_pos = waveform_rect.right_top().x - cue.end * waveform_rect.width() / sample_len;

    let end_icon_size = 16.;
    let end_icon_rect = Rect {
        min: Pos2 {
            x: end_pos - end_icon_size / 2.,
            y: top - end_icon_size,
        },
        max: Pos2 {
            x: end_pos + end_icon_size / 2.,
            y: top + end_icon_size,
        },
    };

    let end_icon_resp = ui.put(
        end_icon_rect,
        egui::Image::new(egui::include_image!("../../../assets/left-and-right.png"))
            .sense(Sense::drag()),
    );

    if end_icon_resp.dragged() {
        let delta = end_icon_resp.drag_delta().x / horiz_scale;
        let new_pos = (cue.end - delta).clamp(0., sample_len - cue.start);
        let _ = cue.set_end(new_pos);
    }

    painter.add(egui::Shape::line_segment(
        [
            Pos2 { x: end_pos, y: top },
            Pos2 {
                x: end_pos,
                y: bottom,
            },
        ],
        Stroke::new(1.5, Color32::BLUE),
    ));
    painter.rect_filled(
        egui::Rect {
            min: Pos2 { x: end_pos, y: top },
            max: waveform_rect.right_bottom(),
        },
        0.,
        Color32::from_rgba_unmultiplied(0, 0, 200, 32),
    );

    Ok(())
}

fn decode_source(cue: &AudioCue, ui: &mut egui::Ui) -> Result<AudioData, anyhow::Error> {
    // let audio_buf_id = Id::new(format!("audio_cue_{}_buf", cue.id));
    // let audio_rate_id = Id::new(format!("audio_cue_{}_rate", cue.id));

    let id = Id::new(format!("audio_cue_{}_data", cue.id));

    // let mut buf: Option<Arc<Mutex<Vec<i16>>>> = None;
    // let mut rate: Option<u32> = None;
    let mut stored: Option<AudioData> = None;
    ui.memory(|mem| {
        // buf = mem.data.get_temp(audio_buf_id);
        // rate = mem.data.get_temp(audio_rate_id);
        stored = mem.data.get_temp(id);
    });
    // if let Some(buf) = buf {
    //     if let Some(rate) = rate {
    //         // no need to recalculate!
    //         return Ok((buf, rate));
    //     }
    // }
    if let Some(out) = stored {
        return Ok(out);
    }

    // info!("decoding");

    let file = BufReader::new(File::open(cue.file_path.clone())?);

    let source = Decoder::new(file)?;

    let rate = source.sample_rate();
    let duration = source.total_duration();

    let out = Arc::new(Mutex::new(source.collect()));

    let data = AudioData {
        samples: out,
        rate,
        duration,
    };

    ui.memory_mut(|mem| {
        // mem.data.insert_temp(audio_buf_id, out.clone());
        // mem.data.insert_temp(audio_rate_id, sample_rate);
        mem.data.insert_temp(id, data.clone());
    });

    Ok(data)
}
