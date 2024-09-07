use eframe::egui::{self, Rect};
use crate::ui_components::colors;
use objective_map_core::{Objective, ObjectiveState};


pub struct ObjectiveWidget {
    pub objective: Objective,
    pub rect: egui::Rect,
}

impl ObjectiveWidget {
    pub fn new(objective: Objective, rect: egui::Rect) -> Self {
        Self {
            objective,
            rect,
        }
    }

    pub fn get_objective_color(&self, state: &ObjectiveState) -> egui::Color32 {
        match state {
            ObjectiveState::Inaccessible => colors::INACCESSIBLE,
            ObjectiveState::Pending => colors::PENDING,
            ObjectiveState::InProgress => colors::IN_PROGRESS,
            ObjectiveState::Complete => colors::COMPLETE,
        }
    }

    pub fn display(&self, painter: &egui::Painter, canvas_pos: egui::Vec2) {
        let rect_pos = egui::Rect {
            min: self.rect.min + canvas_pos,
            max: self.rect.max + canvas_pos,
        };
        // Dessiner le fond du widget avec des coins arrondis
        painter.rect_filled(
            rect_pos,
            egui::Rounding::same(10.0),             // Arrondir les coins
            self.get_objective_color(&self.objective.state), // Couleur de fond
        );

        // Dessiner les bordures du cadre
        painter.rect_stroke(
            rect_pos,
            egui::Rounding::same(10.0),             // Arrondir les coins de la bordure
            egui::Stroke::new(2.0, egui::Color32::BLACK), // Épaisseur et couleur de la bordure
        );

        // Définir la position du texte (centré dans le cadre)
        let text_pos = rect_pos.min + egui::Vec2::new(rect_pos.size().x / 2.0 - 50.0, rect_pos.size().y / 2.0 - 10.0);

        // Dessiner le titre de l'objectif au milieu du widget
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,      // Centrer le texte dans le cadre
            &self.objective.title,
            egui::FontId::proportional(20.0),       // Taille et style de la police
            egui::Color32::BLACK,                   // Couleur du texte
        );
    }
}