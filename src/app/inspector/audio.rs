use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use egui::{Color32, Id, Pos2, Stroke};
// use log::error;
use rodio::{Decoder, Source};

use crate::cues::AudioCue;

use super::CueInspector;

#[derive(Debug)]
pub struct AudioCueInspector<'a> {
    pub cue: &'a mut AudioCue,
}

impl CueInspector for AudioCueInspector<'_> {
    fn basics(&mut self, ui: &mut egui::Ui) -> () {
        ui.horizontal(|ui| {
            ui.label("File: ");
            ui.text_edit_singleline(&mut self.cue.file_path);
        });
    }

    fn time_and_loops(&self) -> bool {
        true
    }

    fn time_and_loops_fn(&mut self, ui: &mut egui::Ui) -> () {
        // ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let total_width = ui.available_width();

            // left column
            ui.vertical(|ui| {
                ui.set_width(total_width * 0.3);

                // position in sink
                ui.horizontal(|ui| {
                    ui.label("Position: ");
                    if let Some(sink) = &self.cue.sink {
                        if sink.empty() {
                            ui.label("Not playing");
                        } else {
                            ui.label(sink.get_pos().as_secs_f64().to_string());
                            // ui.ctx().request_repaint();
                        }
                    } else {
                        ui.label("Cue not initialized!");
                    }
                });
            });

            // right column
            ui.vertical(|ui| {
                // sample views
                draw_waveform_view(&self.cue, ui, total_width)
                // .unwrap_or_else(|err| error!("Error drawing audio waveform: {}", err))
            });
        });
    }
}

fn draw_waveform_view(
    cue: &AudioCue,
    ui: &mut egui::Ui,
    total_width: f32,
) -> Result<(), anyhow::Error> {
    let (mutex, sample_rate) = decode_source(cue, ui)?;
    let samples = match mutex.lock() {
        Err(err) => return Err(anyhow!(err.to_string())),
        Ok(ok) => ok,
    };

    let sink = match &cue.sink {
        None => return Err(anyhow!("Cue had no sync!")),
        Some(s) => s,
    };

    let sample_len = (samples.len() as f32) / (2. * sample_rate as f32);

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
    let displayed_waveform = &samples[..];

    let mut waveform_rect = ui.available_rect_before_wrap();
    waveform_rect.set_width(total_width * 0.7);
    let painter = ui.painter_at(waveform_rect);
    painter.rect_filled(waveform_rect, 4., Color32::BLACK);

    if !displayed_waveform.is_empty() {
        let wave_height = waveform_rect.height() / (2. * 100000.);
        let wave_width = waveform_rect.width() / displayed_waveform.len().max(1) as f32;
        let center_y = waveform_rect.center().y;

        let points: Vec<Pos2> = displayed_waveform
            .iter()
            .enumerate()
            .map(|(i, sample)| {
                let x = waveform_rect.left_top().x + (i as f32 * wave_width);
                let y = center_y - ((*sample as f32) * wave_height);
                Pos2 { x, y }
            })
            .collect();

        painter.add(egui::Shape::line(
            points,
            Stroke::new(1.5, Color32::LIGHT_BLUE),
        ));

        if !sink.empty() {
            let time = sink.get_pos().as_secs_f32();
            let playhead_pos = time * waveform_rect.width() / sample_len;
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

    Ok(())
}

fn decode_source(
    cue: &AudioCue,
    ui: &mut egui::Ui,
) -> Result<(Arc<Mutex<Vec<i16>>>, u32), anyhow::Error> {
    let audio_buf_id = Id::new(format!("audio_cue_{}_buf", cue.id));
    let audio_rate_id = Id::new(format!("audio_cue_{}_rate", cue.id));

    let mut buf: Option<Arc<Mutex<Vec<i16>>>> = None;
    let mut rate: Option<u32> = None;
    ui.memory(|mem| {
        buf = mem.data.get_temp(audio_buf_id);
        rate = mem.data.get_temp(audio_rate_id);
    });
    if let Some(buf) = buf {
        if let Some(rate) = rate {
            // no need to recalculate!
            return Ok((buf, rate));
        }
    }

    // info!("decoding");

    let file = BufReader::new(File::open(cue.file_path.clone())?);

    let source = Decoder::new(file)?;

    let sample_rate = source.sample_rate();
    let out = Arc::new(Mutex::new(source.collect()));

    ui.memory_mut(|mem| {
        mem.data.insert_temp(audio_buf_id, out.clone());
        mem.data.insert_temp(audio_rate_id, sample_rate);
    });

    Ok((out, sample_rate))
}
