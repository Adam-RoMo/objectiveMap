use eframe::egui::{self};
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

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.title);
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Guides", |ui| {
                        if ui.button("Nouveau guide").clicked() {
                            println!("New Guide");
                        }
                        if ui.button("Importer un guide").clicked() {
                            println!("Import a guide");
                        }
                        if ui.button("Exporter un guide").clicked() {
                            println!("Export a guide");
                        }
                        // for guide in guides {}
                    });
                    if ui.button("Mes objectifs").clicked() {
                        println!("Open objectifs panel");
                    }
                    if ui.button("Variables").clicked() {
                        println!("Open variables panel");
                    }
                })
            });
        });
    }
}