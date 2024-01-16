#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod data;
pub mod logging;
mod catch_handler;

use std::sync::{Arc, Mutex};
use std::thread;
use chrono::{Duration, Local};
use egui::{Context, IconData, SidePanel, Vec2};
use rand::{Rng, thread_rng};
use crate::catch_handler::CatchData;
use crate::data::fish::{Fish, FishData};
use crate::data::{fish_data, rod_data};
use crate::data::rods::RodData;
use crate::data::userfile::read_userfile;

const WEIGHT_ADD_TIME: f32 = 0.05;

struct MiniFisher {
    title: String,

    show_side_panel: bool,

    shop_button_content: String,

    rod_data: RodData,
    fish_data: FishData,

    catch_data_ref: Arc<Mutex<CatchData>>,

    dark_theme: bool,
}

impl MiniFisher {
    fn new(title: String) -> Self {
        let catch_data = Arc::new(Mutex::new(CatchData {
            ctx: None,
            cast: false,
            cast_time: None,
            cast_duration: None,
            cast_btn_txt: "Cast rod".to_string(),
            display_text: "Click the button to fish!".to_string(),
            fish: None,
            will_catch: false,

            caught: false,

            running: true,
        }));
        let catch_data_ref = catch_data.clone();

        thread::spawn(move || {
            catch_handler::schedule(catch_data);
        });

        Self {
            title,
            show_side_panel: false,
            shop_button_content: "Shop >".to_string(),

            rod_data: rod_data(),
            fish_data: fish_data(),

            catch_data_ref,

            dark_theme: true,
        }
    }

    fn get_current_catch_data(&self) -> CatchData {
        self.catch_data_ref.lock().unwrap().clone()
    }

    fn cast_rod(&mut self) {
        let mut catch_data = self.catch_data_ref.lock().unwrap();

        // get the user file
        let user_file = read_userfile();

        let rod = user_file.get_rod(&self.rod_data);

        // todo: turtle 🐢🐢🐢🐢
        catch_data.display_text = format!("You cast your {}!", rod);

        // get the fish
        let fish = Fish::random_fish(&self.fish_data, &rod);

        catch_data.cast = true;
        catch_data.caught = false;
        catch_data.cast_btn_txt = "Rod is cast!".to_string();

        // set the cast time
        catch_data.cast_time = Some(Local::now());

        // set the cast duration
        let weight_catch_time_add = (fish.weight - fish.fish_type.avg_weight as f32) * WEIGHT_ADD_TIME;
        let duration = ((rod.random_catch_time() + weight_catch_time_add) * 1000.0) as i64;

        catch_data.cast_duration = Some(Duration::milliseconds(duration));

        catch_data.will_catch = thread_rng().gen_range(0..1000) <= (rod.get_catch_chance());

        catch_data.fish = Some(fish);

        say!("Cast | will catch: {} | duration: {}ms", catch_data.will_catch, duration);
    }

    fn exit(&mut self) {
        let mut catch_data = self.catch_data_ref.lock().unwrap();
        catch_data.running = false;
    }
}

impl eframe::App for MiniFisher {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let theme_btn_text = if self.dark_theme {
            ctx.set_visuals(egui::Visuals::dark());
            "☀"
        } else {
            ctx.set_visuals(egui::Visuals::light());
            "🌙"
        };

        if ctx.input(|i| i.viewport().close_requested()) {
            self.exit();
        };

        let mut catch_data = self.get_current_catch_data();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.shop_button_content = if self.show_side_panel {
                String::from("Shop >")
            } else {
                String::from("Shop <")
            };

            ui.horizontal_top(|ui| {
                let theme_button = ui.button(theme_btn_text).on_hover_text("Click to change theme!");
                if theme_button.clicked() {
                    self.dark_theme = !self.dark_theme;
                }
                let shop_button = ui.button(self.shop_button_content.clone()).on_hover_text("Click to view the shop!");
                if shop_button.clicked() {
                    self.show_side_panel = !self.show_side_panel;
                }
            });

            ui.vertical_centered(|ui| {
                ui.heading(self.title.clone());

                ui.spacing_mut().item_spacing.y = 20.0;

                // display the image
                let img = if self.dark_theme {
                    if catch_data.caught {
                        egui::Image::new(egui::include_image!("../assets/rod_with_fish.png"))
                            .max_size(Vec2::new(64.0, 64.0))
                    } else {
                        egui::Image::new(egui::include_image!("../assets/rod.png"))
                            .max_size(Vec2::new(64.0, 64.0))
                    }
                } else {
                    if catch_data.caught {
                        egui::Image::new(egui::include_image!("../assets/rod_with_fish_darker.png"))
                            .max_size(Vec2::new(64.0, 64.0))
                    } else {
                        egui::Image::new(egui::include_image!("../assets/rod_darker.png"))
                            .max_size(Vec2::new(64.0, 64.0))
                    }
                };

                ui.add(img);

                // display the text
                ui.label(catch_data.display_text.clone());

                // button
                let fish_button = egui::Button::new(catch_data.cast_btn_txt.clone());
                let fish_button_ui = ui.add_enabled(!catch_data.cast, fish_button);
                if fish_button_ui.clicked() {
                    self.cast_rod();
                    self.catch_data_ref.lock().unwrap().ctx = Some(ctx.clone());
                }

                let rod = read_userfile().get_rod(&self.rod_data);

                fish_button_ui.on_hover_text(format!("Rod: {}\nAverage Catch Rate: {}s\nCatch Chance: {}\n\
                Depth: {}ft\nWeight: {}lbs",
                    rod, rod.get_catch_rate(), rod.get_catch_chance() / 100, rod.get_depth(), rod.get_weight_limit()));

                let userfile = read_userfile();

                ui.label(format!("Balance: {}", userfile.money));
                ui.label(format!("Fish Caught: {}", userfile.fish_caught));
                ui.label(format!("Unique Fish: {}/{}", userfile.has_seen.len(), self.fish_data.fish.len()));
            });

            // Show/hide side panel based on button click
            if self.show_side_panel {
                SidePanel::right("shop")
                    .resizable(false)
                    .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_min_size(Vec2::new(395.0, 0.0));
                        ui.heading("Shop");
                        // todo: display shop here
                        ui.spacing_mut().item_spacing.y = 10.0;
                        // Display 5 cards in the side panel
                        // todo: replace with shop items
                        for card_index in 0..6 {
                            ui.horizontal(|ui| {
                                // Each "card" is represented by a button
                                let card_button = ui.button(format!("Card {}", card_index));
                                if card_button.clicked() {
                                    // Todo: shop click
                                }
                            });
                        }
                    });
                });
            }
        });
    }
}

fn load_icon() -> IconData {
    let img_bytes = include_bytes!("../assets/rod.png");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(img_bytes)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_resizable(false)
            .with_minimize_button(false)
            .with_maximize_button(false)
            .with_icon(load_icon()),
        ..Default::default()
    };

    let version = env!("CARGO_PKG_VERSION");
    let app_name = format!("Mini Fisher v{}", version);

    if let Err(e) = eframe::run_native(
        app_name.clone().as_str(),
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MiniFisher::new(app_name))
        }),
    ) {
        eprintln!("Error: {}", e);
    }
}
