use eframe::egui::{self, Color32, Response, Ui};

pub struct PanelButton {
    pub label: String,
    pub color: Color32,
}

impl PanelButton {
    pub fn new(label: &str, color: Color32) -> Self {
        Self {
            label: label.to_string(),
            color,
        }
    }

    pub fn ui(&self, ui: &mut Ui) -> Response {
        ui.add(
            egui::Button::new(&self.label)
                .fill(self.color)
                .rounding(5.0),
        )
    }
}