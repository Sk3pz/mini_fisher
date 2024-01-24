use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Local};
use rand::{Rng, thread_rng};
use crate::data::fish::Fish;
use crate::data::userfile::update_userfile;
//use crate::say;

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
    pub was_turtle: bool,

    pub running: bool
}

pub fn reset(data: &mut CatchData) {
    data.cast = false;
    data.fish = None;
    data.will_catch = false;
    data.cast_time = None;
    data.cast_duration = None;
    data.cast_btn_txt = "Cast rod".to_string();
    //say!("Reeled!");
    if let Some(ctx) = &data.ctx {
        ctx.request_repaint();
        data.ctx = None;
    }
}

pub fn schedule(data: Arc<Mutex<CatchData>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let mut data = data.lock().unwrap();

        if !data.running {
            continue;
        }

        if data.cast {
            data.was_turtle = false;

            // check if the cast has expired
            let current_elapsed = Local::now().signed_duration_since(data.cast_time.unwrap());
            let cast_duration = data.cast_duration.unwrap();

            if current_elapsed < cast_duration {
                continue;
            }

            let fish = data.fish.clone().unwrap();

            let userfile = crate::read_userfile();

            let rod_data = crate::rod_data();

            let rod = userfile.get_rod(&rod_data);

            drop(rod_data); // free up the wasted memory

            // check if the fish is too heavy
            if fish.weight as u32 > rod.get_weight_limit() {
                data.display_text = format!("Your line broke! The {}lb {} was too heavy!", fish.weight, fish);

                data.caught = false;
                reset(&mut data);
                continue;
            }

            if data.will_catch {
                // turtle event 🐢🐢🐢
                if thread_rng().gen_range(0..100) >= 98 {
                    data.display_text = format!("🐢 A turtle stole your {}lb {}! 🐢", fish.weight, fish);
                    data.caught = false;
                    data.was_turtle = true;
                    reset(&mut data);
                    continue;
                }

                let fishdata = crate::data::fish_data();
                let value = fish.get_value(&fishdata);

                data.display_text = format!("You caught a ${} {} at {}lbs!", value, fish, fish.weight);
                data.caught = true;
                // update userfile
                let mut userfile = crate::read_userfile();

                userfile.fish_caught += 1;
                userfile.money += value;
                if !userfile.has_seen.contains(&fish.fish_type.name) {
                    userfile.has_seen.push(fish.fish_type.name.clone());
                }
                update_userfile(userfile);
            } else {
                data.display_text = format!("A {}lbs {} got away! Better luck next time!", fish.weight, fish);
                data.caught = false;
            }

            reset(&mut data);
        }
    }
}