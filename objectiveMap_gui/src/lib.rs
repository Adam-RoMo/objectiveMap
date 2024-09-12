mod ui_components;

use eframe::egui::{self, viewport, Margin, Rounding};
use ui_components::{MovableCanvas, TopPanel, ObjectiveInfoWindow, ObjectivesPanel};
use objective_map_core::{objective::{self, Vec2}, Guide, ObjectiveState};


struct ObjectiveApp {
    guide: Guide,
    top_panel: TopPanel,
    guide_canvas: MovableCanvas,
    objective_info: ObjectiveInfoWindow,
    objectives_panel: ObjectivesPanel,
    edit_mode: bool,
    // variables_panel: VariablesPanel,
    // frame_style: egui::Style,
}

impl Default for ObjectiveApp {
    fn default() -> Self {
        let mut guide = Guide::new("Mon Guide", "C'est un super guide");
        guide.add_objective("Premier Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::InProgress);
        // guide.add_objective("Deuxième Objectif", "Une superbe description pour un superbe objectif", ObjectiveState::InProgress);
        Self {
            guide: guide,
            top_panel: TopPanel::new("Mon Panel Top"),
            guide_canvas: MovableCanvas::new(),
            objective_info: ObjectiveInfoWindow::new(),
            objectives_panel: ObjectivesPanel::new(),
            edit_mode: false,
            // frame_style: style,
        }
    }
}

impl eframe::App for ObjectiveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel.ui(ctx, &mut self.guide);
        
        egui::CentralPanel::default().frame(egui::Frame {
            stroke: egui::Stroke::NONE,
            ..Default::default()
        }).show(ctx, |ui| {
            self.guide_canvas.ui(ui, &mut self.guide, self.edit_mode);
        });

        // if  
        self.objectives_panel.ui(ctx, &mut self.guide);
        // egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
        //     ui.horizontal_wrapped(|ui| {
        //         egui::Frame::group(ui.style()).show(ui, |ui| {
        //             ui.label("Contenu 1");
        //             ui.label("Contenu 2");
        //             ui.label("Contenu 3");
        //             ui.label("Contenu 4");
        //         });          
        //     });
        // });

        self.objective_info.ui(ctx, &mut self.guide);

        egui::TopBottomPanel::top("button_panel").frame(egui::Frame::none())
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Edit Mode").clicked() {
                        self.edit_mode = !self.edit_mode;
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