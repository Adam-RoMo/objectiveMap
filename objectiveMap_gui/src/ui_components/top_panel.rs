use crate::ui_components::PanelButton;
use eframe::egui::{self, Color32};
use crate::ui_components::colors;

use super::PanelPopup;


pub struct TopPanel {
    pub title: String,
    guides_popup: PanelPopup,
    guides_button: PanelButton,
    objectifs_button: PanelButton,
    variables_button: PanelButton
}

impl TopPanel {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            guides_popup: PanelPopup::new("GuidesPopup", colors::PRIMARY_COLOR),
            guides_button: PanelButton::new("GuidesButton", colors::PRIMARY_COLOR),
            objectifs_button: PanelButton::new("ObjectifsButton", colors::PRIMARY_COLOR),
            variables_button: PanelButton::new("VaraiblesButton", colors::PRIMARY_COLOR)
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.title);
                self.guides_popup.ui(ui, ctx);
                // if self.guides_button.ui(ui).clicked() {
                //     println!("Guides");
                // }
                // if self.objectifs_button.ui(ui).clicked() {
                //     println!("Objectifs");
                // }
                // if self.variables_button.ui(ui).clicked() {
                //     println!("Variables");
                // }
            });
        });
    }
}