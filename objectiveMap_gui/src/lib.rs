mod ui_components;

use eframe::egui::{self, viewport, Margin, Rounding};
use ui_components::{MovableCanvas, TopPanel};
use objective_map_core::{objective::Vec2, Guide, ObjectiveState};


struct ObjectiveApp {
    guide: Guide,
    top_panel: TopPanel,
    guide_canvas: MovableCanvas,
    // objectifs_panel: ObjectifsPanel,
    // variables_panel: VariablesPanel,
    // frame_style: egui::Style,
}

impl Default for ObjectiveApp {
    fn default() -> Self {
        let mut guide = Guide::new("Mon Guide", "C'est un super guide");
        let pre = guide.add_objective("Premier Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::Inaccessible, Vec2{x: 50.0, y: 100.0});
        let dep = guide.add_objective("Deuxième Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::InProgress, Vec2{x: 500.0, y: 500.0});
        guide.connect_objectives(pre, dep, "Test relation");
        Self {
            guide: guide,
            top_panel: TopPanel::new("Mon Panel Top"),
            guide_canvas: MovableCanvas::new(),
            // frame_style: style,
        }
    }
}

impl eframe::App for ObjectiveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_debug_on_hover(true);
        self.top_panel.ui(ctx);
        
        egui::CentralPanel::default().frame(egui::Frame {
            stroke: egui::Stroke::NONE,
            ..Default::default()
        }).show(ctx, |ui| {
            self.guide_canvas.ui(ui, &mut self.guide);
        });

        // if 
        egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label("Contenu 1");
                    ui.label("Contenu 2");
                    ui.label("Contenu 3");
                    ui.label("Contenu 4");
                });            
            });
        });

        egui::TopBottomPanel::top("button_panel").frame(egui::Frame {
            fill: egui::Color32::TRANSPARENT, // Définit le fond transparent
            stroke: egui::Stroke::NONE,     // Pas de bordure
            shadow: egui::Shadow::NONE,
            inner_margin: Margin::ZERO,
            outer_margin: Margin::ZERO,
            rounding: Rounding::ZERO
        }).max_height(0.0) // Définir une hauteur minimale pour que le panel ne prenne pas d'espace
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Bouton").clicked() {
                        // Action du bouton
                    }
                });
            });
        });
    }
}

pub fn run_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: viewport::ViewportBuilder::with_inner_size(viewport::ViewportBuilder::default(), egui::Vec2::new(1600.0, 1000.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My ObjectiveMap App",
        options,
        Box::new(|_cc| Ok(Box::new(ObjectiveApp::default()))),
    )
}