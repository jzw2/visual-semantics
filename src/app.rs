use crate::ast::Rule;
use crate::ast::Stack;
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Debug)]
pub struct TemplateApp {
    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    my_enum: Rule,
    current_display_text: String,
    stack: Stack,
}

impl Rule {
    fn to_string(&self) -> String {
        println!("{:?}", self);
        let x = match self {

            Rule::RewriteVariableLookup => "crl o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .".to_string(),
            Rule::RewritePlusLeft => "crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
            Rule::RewritePlusRight => "crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
            Rule::RewritePlus => " rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .".to_string(),

            Rule::RewriteDivideLeft => "crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
            Rule::RewriteDivideRight => "crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
            Rule::RewriteDivide => "crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .".to_string(),
        
            Rule::RewriteLessThanLeft => "crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .".to_string(),
            Rule::RewriteLessThanRight => "crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .".to_string(),
            Rule::RewriteLessThan => "rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .".to_string(),
            Rule::RewriteNegate => "crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .".to_string(),
            Rule::RewriteNegateTrue => "rl o < ! true,Sigma > => < false,Sigma > .".to_string(),
            Rule::RewriteNegateFalse => "rl o < ! false,Sigma > => < true,Sigma > .".to_string(),
            
            Rule::RewriteBlockStatement => "rl o < {S},Sigma > => < S,Sigma > .".to_string(),
            Rule::RewriteAssignmentArith => "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .".to_string(),
            Rule::RewriteAssignmentInt => "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .".to_string(),

            Rule::RewriteSequence => "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > .".to_string(),
            Rule::RewriteEmptyBlock => "rl o < {} S2,Sigma > => < S2,Sigma > .".to_string(),
            
            Rule::RewriteConditional => "crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .".to_string(),
            Rule::RewriteConditionalTrue => "rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .".to_string(),
            Rule::RewriteConditionalFalse => "rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .".to_string(),

            Rule::RewriteLoop => "rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .".to_string(),
            Rule::NoOp => "None selected".to_string(),
            _ => "".to_string(),
        };
        println!("x is {:?}", x);
        x
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            my_enum: Rule::NoOp,
            current_display_text: "".to_string(),
            stack: Stack::new(),
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            label: _,
            value: _,
            my_enum,
            current_display_text: _,
            stack,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").min_width(500.0).show(ctx, |ui| {


            ui.radio_value(my_enum, Rule::RewriteVariableLookup, "crl o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .");
            ui.radio_value(my_enum, Rule::RewritePlusLeft, "crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            ui.radio_value(my_enum, Rule::RewritePlusRight, "crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            ui.radio_value(my_enum, Rule::RewritePlus, " rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .");

            ui.radio_value(my_enum, Rule::RewriteDivideLeft, "crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteDivideRight, "crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteDivide, "crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .");
            
        
            ui.radio_value(my_enum, Rule::RewriteLessThanLeft, "crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteLessThanRight, "crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteLessThan, "rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteNegate, "crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteNegateTrue, "rl o < ! true,Sigma > => < false,Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteNegateFalse, "rl o < ! false,Sigma > => < true,Sigma > .");
            
            ui.radio_value(my_enum, Rule::RewriteBlockStatement, "rl o < {S},Sigma > => < S,Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteAssignmentArith, "crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteAssignmentInt, "crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .");

            ui.radio_value(my_enum, Rule::RewriteSequence, "crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > .");
            ui.radio_value(my_enum, Rule::RewriteEmptyBlock, "rl o < {} S2,Sigma > => < S2,Sigma > .");
            
            ui.radio_value(my_enum, Rule::RewriteConditional, "crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .");
            ui.radio_value(my_enum, Rule::RewriteConditionalTrue, "rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .");
            ui.radio_value(my_enum, Rule::RewriteConditionalFalse, "rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .");

            ui.radio_value(my_enum, Rule::RewriteLoop, "rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .");


            ui.radio_value(my_enum, Rule::RewriteTop, "rl o < int Xl ; S > => < S,(Xl |-> 0) > .");

            // ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     ui.horizontal(|ui| {
            //         ui.spacing_mut().item_spacing.x = 0.0;
            //         ui.label("powered by ");
            //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //         ui.label(" and ");
            //         ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
            //     });
            // });
            //
            // ui.label("crl o < X,Sigma > => < Sigma(X),Sigma > if Sigma(X) =/=Bool undefined .");
            // ui.label("crl o < A1 + A2,Sigma > => < A1' + A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            // ui.label("crl o < A1 + A2,Sigma > => < A1 + A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            // ui.label("rl o < I1 + I2,Sigma > => < I1 +Int I2,Sigma > .");
            // ui.label("crl o < A1 / A2,Sigma > => < A1' / A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            // ui.label("crl o < A1 / A2,Sigma > => < A1 / A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            // ui.label("crl o < I1 / I2,Sigma > => < I1 /Int I2,Sigma > if I2 =/=Bool 0 .");
            // ui.label("crl o < A1 <= A2,Sigma > => < A1' <= A2,Sigma > if o < A1,Sigma > => < A1',Sigma > .");
            // ui.label("crl o < I1 <= A2,Sigma > => < I1 <= A2',Sigma > if o < A2,Sigma > => < A2',Sigma > .");
            // ui.label("rl o < I1 <= I2,Sigma > => < I1 <=Int I2,Sigma > .");
            // ui.label("crl o < ! B,Sigma > => < ! B',Sigma > if o < B,Sigma > => < B',Sigma > .");
            // ui.label("rl o < ! true,Sigma > => < false,Sigma > .");
            // ui.label("rl o < ! false,Sigma > => < true,Sigma > .");
            // ui.label("crl o < B1 && B2,Sigma > => < B1' && B2,Sigma > if o < B1,Sigma > => < B1',Sigma > .");
            // ui.label("rl o < false && B2,Sigma > => < false,Sigma > .");
            // ui.label("rl o < true && B2,Sigma > => < B2,Sigma > .");
            // ui.label("rl o < {S},Sigma > => < S,Sigma > .");
            // ui.label("crl o < if (B) S1 else S2,Sigma > => < if (B') S1 else S2,Sigma > if o < B,Sigma > => < B',Sigma  > .");
            // ui.label("rl o < if (true) S1 else S2,Sigma > => < S1,Sigma > .");
            // ui.label("rl o < if (false) S1 else S2,Sigma > => < S2,Sigma > .");
            // ui.label("rl o < while (B) S,Sigma > => < if (B) {S while (B) S} else {},Sigma > .");

            // ui.label("rl o < {} S2,Sigma > => < S2,Sigma > .");
            // ui.label("crl o < S1 S2,Sigma > => < S1' S2,Sigma' > if o < S1,Sigma > => < S1',Sigma' > .");
            // ui.label("crl o < X = A ;,Sigma > => < X = A' ;,Sigma > if o < A,Sigma > => < A',Sigma > .");
            // ui.label("crl o < X = I ;,Sigma > => < {},Sigma[I / X] > if Sigma(X) =/=Bool undefined .");
            // ui.label("rl o < int Xl ; S > => < S,(Xl |-> 0) > .");


            if ui.button("Apply").clicked() {
                stack.applyRule(my_enum.clone());
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            // egui::warn_if_debug_build(ui);

            //println!("{:?}", current_display_text);
            ui.label(format!("{}", stack));
        });
    }
}
