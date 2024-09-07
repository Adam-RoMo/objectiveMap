mod ui_components;

use eframe::egui::{self, viewport, Vec2};
use ui_components::{MovableCanvas, TopPanel};
// use objective_map_core::{guide, objective};


struct ObjectiveApp {
    top_panel: TopPanel,
    guide_canvas: MovableCanvas,
    // objectifs_panel: ObjectifsPanel,
    // variables_panel: VariablesPanel,
    // frame_style: egui::Style,
    // guide: Guide,
}

impl Default for ObjectiveApp {
    fn default() -> Self {
        // let mut guide = 
        Self {
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
            self.guide_canvas.ui(ui);
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
        viewport: viewport::ViewportBuilder::with_inner_size(viewport::ViewportBuilder::default(), Vec2::new(1600.0, 1000.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My ObjectiveMap App",
        options,
        Box::new(|_cc| Ok(Box::new(ObjectiveApp::default()))),
    )
}