mod ui_components;
use eframe::egui;
use ui_components::TopPanel;
// use objective_map_core::{guide, objective};


struct ObjectiveApp {
    top_panel: TopPanel,
    // guide: Guide,
}

impl Default for ObjectiveApp {
    fn default() -> Self {
        // let mut guide = 
        Self {
            top_panel: TopPanel::new("Mon Panel Top"),
        }
    }
}

impl eframe::App for ObjectiveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.top_panel.ui(ctx);
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Contenu principal de l'application");
            });
        });
    }
}

pub fn run_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My ObjectiveMap App",
        options,
        Box::new(|_cc| Ok(Box::new(ObjectiveApp::default()))),
    )
}