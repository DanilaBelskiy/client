use egui::{SliderOrientation, Ui, Vec2};
use egui::style::Spacing;
use egui::Label;

use client_server::GroupToRotate;

use std::fs::File;
use std::io::Write;
use std::string::String;

use serde::__private::de::Content::String as OtherString;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    corner_a: f32,
    corner_b: f32,
    corner_c: f32,
    corner_d: f32,

    check_box: bool,

    login_str: String,
    password_str: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            corner_a: 0.0,
            corner_b: 0.0,
            corner_c: 0.0,
            corner_d: 0.0,
            check_box: false,
            login_str: String::new(),
            password_str: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label,
            corner_a,
            corner_b,
            corner_c,
            corner_d,
            check_box,
            login_str,
            password_str,
        } = self;

        let mut output = File::create("current_corners.txt").unwrap();
        write!(output, "{}\n{}\n{}\n{}", corner_a, corner_b, corner_c, corner_d).unwrap();

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Server");

            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.checkbox(check_box, "Real Time");

            if *check_box == false {
                ui.horizontal(|ui| {
                    if ui.button("MOVE").clicked() {
                        //pass
                    }
                    if ui.button("STOP").clicked() {
                        //pass
                    }
                });
                if ui.button("READ POSITION").clicked() {
                    //pass
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {

                if ui.button("Log in").clicked() {
                    // logging in
                };

                ui.horizontal(|ui| {
                    ui.label("Login: ");
                    ui.text_edit_singleline(login_str);
                });

                ui.horizontal(|ui| {
                    ui.label("Password: ");
                    ui.text_edit_singleline(password_str);
                });

                ui.label("");

                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Axes of rotation");

            ui.spacing_mut().slider_width = 200.0;
            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.add( egui::Slider::new(corner_a, -1.5..=1.5).text("Axis A"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_a = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_a = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_a += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_a -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_b, -1.5..=1.5).text("Axis B"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_b = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_b = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_b += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_b -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_c, -1.5..=1.5).text("Axis C"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_c = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_c = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_c += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_c -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_d, -3.3..=3.3).text("Axis D"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_d = 3.3;

                }
                if ui.button("MIN").clicked() {
                    *corner_d = -3.3;

                }
                if ui.button("+").clicked() {
                    *corner_d += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_d -= 0.1;
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}