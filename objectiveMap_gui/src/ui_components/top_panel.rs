use eframe::egui::{self};
use objective_map_core::{self, Guide, ObjectiveState};
use crate::PanelStatus;
use std::fs::File;

// use crate::ui_components::colors;


pub struct TopPanel {
    pub title: String,
}


impl TopPanel {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, guide: &mut Guide, panel_status: &mut PanelStatus, file_path: &mut Option<String>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.title);
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Guides", |ui| {
                        if ui.button("Nouveau guide").clicked() {
                            // *guide = Guide::new(title, description);
                        }
                        if ui.button("Importer un guide").clicked() {
                            if let Some(new_guide) = Guide::load_guide() {
                                *guide = new_guide;
                            }
                        }
                        if ui.button("Exporter un guide").clicked() {
                            Guide::export_guide(&guide);
                        }
                        if ui.button("Sauvegarder").clicked() {
                            Guide::save_guide(guide, file_path);
                        }
                    });
                    if ui.button("Mes objectifs").clicked() {
                        *panel_status = match *panel_status {
                            PanelStatus::OBJECTIVES => PanelStatus::NONE,
                            PanelStatus::VARIABLES => PanelStatus::OBJECTIVES,
                            PanelStatus::NONE => PanelStatus::OBJECTIVES
                        };
                    }
                    if ui.button("Variables").clicked() {
                        *panel_status = match *panel_status {
                            PanelStatus::OBJECTIVES => PanelStatus::VARIABLES,
                            PanelStatus::VARIABLES => PanelStatus::NONE,
                            PanelStatus::NONE => PanelStatus::VARIABLES
                        };
                        println!("Open variables panel");
                    }
                    if ui.button("Nouvel objectif").clicked() {
                        println!("New objective");

                        guide.add_objective("Nouvel objectif", "Description de l'objectif", ObjectiveState::Pending);
                    }
                })
            });
        });
    }
}