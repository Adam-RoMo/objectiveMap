use eframe::egui::{self, Color32, Sense, Shape, Vec2};

pub struct CircleButton {
    pub position: egui::Pos2,
    pub radius: f32,
    pub color: Color32,
}

impl CircleButton {
    pub fn new(position: egui::Pos2, radius: f32, color: Color32) -> Self {
        Self { position, radius, color }
    }

    pub fn ui<F>(&mut self, ui: &mut egui::Ui, on_click: F) -> egui::Response
    where
        F: FnOnce(),
     {
        // Définir un rectangle autour du cercle pour capturer les interactions
        let rect = egui::Rect::from_center_size(
            self.position,
            Vec2::new(self.radius * 2.0, self.radius * 2.0),
        );

        // Gérer les interactions comme le clic
        let response = ui.allocate_rect(rect, Sense::click());

        // Dessiner le cercle
        let painter = ui.painter();
        painter.add(Shape::circle_filled(
            self.position,
            self.radius,
            self.color,
        ));

        // Si le cercle est cliqué
        if response.clicked() {
            on_click();
        }

        response
    }
}
