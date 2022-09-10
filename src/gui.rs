use egui::{SliderOrientation, Ui, Vec2};
use egui::style::Spacing;

use client_server::GroupToRotate;


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
            check_box} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Axes of rotation");

            ui.spacing_mut().slider_width = 200.0;
            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.add( egui::Slider::new(corner_a, -10.0..=10.0).text("Axis A"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_a = 10.0;

                }
                if ui.button("MIN").clicked() {
                    *corner_a = -10.0;

                }
                if ui.button("+").clicked() {
                    *corner_a += 1.0;

                }
                if ui.button("-").clicked() {
                    *corner_a -= 1.0;
                }
            });
            ui.add( egui::Slider::new(corner_b, -10.0..=10.0).text("Axis B"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_b = 10.0;

                }
                if ui.button("MIN").clicked() {
                    *corner_b = -10.0;

                }
                if ui.button("+").clicked() {
                    *corner_b += 1.0;

                }
                if ui.button("-").clicked() {
                    *corner_b -= 1.0;
                }
            });
            ui.add( egui::Slider::new(corner_c, -10.0..=10.0).text("Axis C"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_c = 10.0;

                }
                if ui.button("MIN").clicked() {
                    *corner_c = -10.0;

                }
                if ui.button("+").clicked() {
                    *corner_c += 1.0;

                }
                if ui.button("-").clicked() {
                    *corner_c -= 1.0;
                }
            });
            ui.add( egui::Slider::new(corner_d, -10.0..=10.0).text("Axis D"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_d = 10.0;

                }
                if ui.button("MIN").clicked() {
                    *corner_d = -10.0;

                }
                if ui.button("+").clicked() {
                    *corner_d += 1.0;

                }
                if ui.button("-").clicked() {
                    *corner_d -= 1.0;
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