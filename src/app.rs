use crate::ast::Rule;
use crate::ast::Stack;

use eframe::{egui, epi};

use egui::FontDefinitions;

use egui::CollapsingHeader;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Debug)]
pub struct TemplateApp {
    // Example stuff:
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    my_enum: Rule,
    current_display_text: String,
    stack: Stack,
    start_program: String,
    error_message: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            my_enum: Rule::NoOp,
            current_display_text: "".to_string(),
            stack: Stack::new(),
            start_program: "int x, y;\n  x = x + 1;".to_string(),
            error_message: "".to_string(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {
            my_enum,
            current_display_text: _,
            stack,
            start_program,
            error_message,
        } = self;

        egui::SidePanel::left("side_panel")
            .min_width(500.0)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 50.0;
                });

                ui.label("Applicable Rules:");

                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    let applicable_rules = Rule::list_of_rules()
                        .into_iter()
                        .filter(|rule| stack.can_apply_rule(rule.clone()));
                    for rule in applicable_rules {
                        let label = rule.get_label();
                        ui.label(label);
                        let description = rule.get_description();
                        ui.radio_value(my_enum, rule, description);
                        ui.end_row();
                    }
                });
                if ui.button("Apply").clicked() {
                    stack.apply_rule(my_enum.clone());
                }

                //     let mut fonts = FontDefinitions::default();

                // // Large button text:
                //     fonts.family_and_size.insert(
                //         TextStyle::Body,
                //         (FontFamily::Proportional, 60.0)
                //     );

                //     ctx.set_fonts(fonts);

                for rule in Rule::list_of_rules() {
                    let label = rule.get_label();
                    CollapsingHeader::new(label)
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(rule.get_description());
                        });
                }
                ctx.set_fonts(FontDefinitions::default());
            });
        // egui::TopBottomPanel::bottom("hi").show(ctx, |ui| {

        // });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Undo").clicked() {
                    stack.undo();
                }
                if ui.button("Redo").clicked() {
                    stack.redo();
                }
            });

            ui.horizontal_wrapped(|ui| {
                if ui.button("Pop Top").clicked() {
                    stack.pop();
                }
                if ui.button("Clear stack").clicked() {
                    stack.clear();
                }
            });
            ui.label(format!("{}", stack));

            let _response = ui.add(egui::TextEdit::multiline(start_program));
            if ui.button("Use new program").clicked() {
                if let Some(s) = Stack::create_from_string(start_program.to_string()) {
                    println!("parsed as {:?}", s);
                    *stack = s;
                    *error_message = "".to_string();
                } else {
                    println!("parse failed");
                    *error_message = "Unable to parse".to_string();
                }
                println!("{:?}", stack);
            }
            ui.label(error_message);
        });
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}
