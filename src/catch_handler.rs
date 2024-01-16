use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Local};
use crate::data::fish::Fish;
use crate::say;

#[derive(Clone)]
pub struct CatchData {
    pub ctx: Option<egui::Context>,

    pub cast: bool,
    pub cast_time: Option<DateTime<Local>>,
    pub cast_duration: Option<Duration>,
    pub cast_btn_txt: String,
    pub display_text: String,
    pub will_catch: bool,
    pub fish: Option<Fish>,

    pub caught: bool,

    pub running: bool
}

pub fn reset(data: &mut CatchData) {
    data.cast = false;
    data.fish = None;
    data.will_catch = false;
    data.cast_time = None;
    data.cast_duration = None;
    data.cast_btn_txt = "Cast rod".to_string();
    data.display_text = "Click the button to fish!".to_string();
    data.caught = true;
}

pub fn schedule(data: Arc<Mutex<CatchData>>) {
    while data.lock().unwrap().running {
        let mut data = data.lock().unwrap();

        if data.cast {

            if data.cast_time.is_none() {
                data.cast_time = Some(Local::now());
            }

            if data.cast_duration.is_none() {
                data.cast_duration = Some(Duration::seconds(20));
            }

            let current_elapsed = Local::now().signed_duration_since(data.cast_time.unwrap());
            let cast_duration = data.cast_duration.unwrap();

            if current_elapsed < cast_duration {
                continue;
            }

            reset(&mut data);
            say!("Reeled!");
            if let Some(ctx) = &data.ctx {
                ctx.request_repaint();
                data.ctx = None;
            }
        }
    }
}