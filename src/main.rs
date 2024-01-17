#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod data;
pub mod logging;
mod catch_handler;

use std::sync::{Arc, Mutex};
use std::thread;
use chrono::{Duration, Local};
use egui::{Context, IconData, Response, SidePanel, Ui, Vec2};
use rand::{Rng, thread_rng};
use crate::catch_handler::CatchData;
use crate::data::fish::{Fish, FishData};
use crate::data::{fish_data, rod_data};
use crate::data::rods::{BaseRod, Rod, RodData};
use crate::data::shop::Shop;
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

    cached_catch_data: Option<CatchData>,
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

            was_turtle: false,
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

            cached_catch_data: None,
        }
    }

    fn cast_rod(&mut self) {
        let mut catch_data = self.catch_data_ref.lock().unwrap();

        // get the user file
        let user_file = read_userfile();

        let rod = user_file.get_rod(&self.rod_data);

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
        let duration = (rod.random_catch_time() + weight_catch_time_add) as i64;

        catch_data.cast_duration = Some(Duration::seconds(duration));

        catch_data.will_catch = thread_rng().gen_range(0..1000) <= (rod.get_catch_chance());

        catch_data.fish = Some(fish);

        say!("Cast | will catch: {} | duration: {}s", catch_data.will_catch, duration);
    }

    fn exit(&mut self) {
        let mut catch_data = self.catch_data_ref.lock().unwrap();
        catch_data.running = false;
    }

    fn generate_shop_and_theme_buttons(&mut self, ui: &mut Ui, theme_btn_text: &str) {
        ui.horizontal(|ui| {
            let theme_button = ui.button(theme_btn_text).on_hover_text("Click to change theme!");
            if theme_button.clicked() {
                self.dark_theme = !self.dark_theme;
            }
            let shop_button = ui.button(self.shop_button_content.clone()).on_hover_text("Click to view the shop!");
            if shop_button.clicked() {
                self.show_side_panel = !self.show_side_panel;
            }
        });
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

        let catch_data = if let Ok(cd) = self.catch_data_ref.try_lock() {
            cd.clone()
        } else {
            if let Some(c) = &self.cached_catch_data {
                c.clone()
            } else {
                self.catch_data_ref.lock().unwrap().clone()
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            self.shop_button_content = if self.show_side_panel {
                String::from("Shop >")
            } else {
                String::from("Shop <")
            };

            // Show/hide side panel based on button click
            if self.show_side_panel {
                let mut shop = Shop::load(&self.rod_data);

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    self.generate_shop_and_theme_buttons(ui, theme_btn_text);
                    let userfile = read_userfile();
                    ui.heading("Balance:");
                    ui.label(format!("${}", userfile.money));
                    let rod = userfile.get_rod(&self.rod_data);
                    ui.heading("Your Rod:");
                    let rod_name = rod.to_string();
                    let rod_name = if rod_name.len() > 25 {
                        let mut rod_name = rod_name.split_at(20).0.to_string();
                        rod_name.push_str("...");
                        rod_name
                    } else {
                        rod_name
                    };
                    let rod_label = ui.label(format!("{}\n(hover for more info)", rod_name));
                    add_hover_txt_mod(rod_label, &rod);
                });

                SidePanel::right("shop")
                    .resizable(false)
                    .show(ctx, |ui| {

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.set_min_size(Vec2::new(325.0, 0.0));
                                ui.spacing_mut().item_spacing.y = 10.0;
                                ui.heading("Shop");
                                ui.label("Hover over an item's buy button to see more information!");
                                ui.label(format!("Next restock in: {}", shop.get_time_until_restock()));

                                for x in 0..shop.rods.len() {
                                    let rod = &shop.rods[x];
                                    let rod = self.rod_data.get_base_by_name(rod).unwrap();

                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            ui.heading(rod.name.clone());
                                            let desc = ui.label(format!("{}\n(Hover for more information)", rod.description));
                                            add_hover_txt(desc, &rod);
                                            ui.label(format!("${}", rod.cost));
                                        });
                                        let buy_button = egui::Button::new("Buy");
                                        let has_money = read_userfile().money >= rod.cost as u32;
                                        let buy_button_ui = ui.add_enabled(has_money, buy_button);
                                        if buy_button_ui.clicked() {
                                            let buy_result = shop.sell_rod(x, &self.rod_data);
                                            if let Err(e) = buy_result {
                                                say!("Failed to buy rod: {:?}", e);
                                            }
                                        }
                                        add_hover_txt(buy_button_ui, &rod);
                                    });
                                }

                                ctx.request_repaint();
                            });
                        });

                });
            } else { // not showing shop side panel
                self.generate_shop_and_theme_buttons(ui, theme_btn_text);

                ui.vertical_centered(|ui| {
                    ui.heading(self.title.clone());

                    ui.spacing_mut().item_spacing.y = 20.0;

                    // display the image
                    let img = if catch_data.was_turtle {
                        egui::Image::new(egui::include_image!("../assets/turtle.png"))
                            .max_size(Vec2::new(64.0, 64.0))
                    } else {
                        if self.dark_theme {
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

                    add_hover_txt_mod(fish_button_ui, &rod);

                    let userfile = read_userfile();

                    ui.label(format!("Balance: {}", userfile.money));
                    ui.label(format!("Fish Caught: {}", userfile.fish_caught));
                    ui.label(format!("Unique Fish: {}/{}", userfile.has_seen.len(), self.fish_data.fish.len()));
                });
            }
        });
    }
}

fn add_hover_txt(response: Response, rod: &BaseRod) {
    response.on_hover_text(format!("{}\nAverage Catch Rate: {}s\n\
                                        Catch Chance: {}%\nDepth: {}ft\nWeight: {}lbs",
                                   rod.name, rod.catch_rate,
                                   (rod.catch_chance * 100.0) as u32, rod.depth,
                                   rod.weight_limit));
}

fn add_hover_txt_mod(response: Response, rod: &Rod) {
    response.on_hover_text(format!("{}\nAverage Catch Rate: {}s\n\
                                        Catch Chance: {}%\nDepth: {}ft\nWeight: {}lbs",
                                   rod, rod.get_catch_rate(),
                                   rod.get_catch_chance() / 10, rod.get_depth(),
                                   rod.get_weight_limit()));
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
