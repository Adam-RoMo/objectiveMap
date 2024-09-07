mod ui_components;

use eframe::egui::{self, viewport};
use ui_components::{MovableCanvas, TopPanel};
use objective_map_core::{objective::Vec2, Guide, Objective, ObjectiveState};


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
        guide.add_objective("Premier Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::Inaccessible, Vec2{x: 50.0, y: 50.0});
        guide.add_objective("Deuxième Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::InProgress, Vec2{x: 50.0, y: 100.0});
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
        self.top_panel.ui(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.guide_canvas.ui(ui, &mut self.guide);
        });

        // if 
        egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label("Contenu 1");
                    ui.label("Contenu 2");
                    ui.label("Contenu 3");
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