use eframe::egui;
use crate::ui_components::colors;
use objective_map_core::{Objective, ObjectiveState};


// const MAX_SIZE: egui::Vec2 = egui::Vec2 {
//     x: 200.0,
//     y: 100.0
// };

const MARGIN: egui::Vec2 = egui::Vec2 {
    x: 5.0,
    y: 3.0
};

pub struct ObjectiveWidget {
    // pub objective: Objective,
    // pub pos: egui::Pos2,
    // pub size: Option<egui::Vec2>,
}

impl ObjectiveWidget {
    pub fn new() -> Self {
        Self {
            // objective,
            // pos,
            // size: None
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

    pub fn display(&self, painter: &egui::Painter, canvas_pos: egui::Vec2, objective: &mut Objective) {
        let text_size: egui::Vec2;

        text_size = match &objective.size {
            Some(n) => egui::Vec2 {x: n.x, y: n.y},
            None => painter.layout_no_wrap(objective.title.to_string(), egui::FontId::proportional(20.0), egui::Color32::WHITE).size()
        };
        let rect_pos = egui::Rect {
            min: egui::Pos2::new(objective.pos.x, objective.pos.y) + canvas_pos - (text_size / 2.0) - MARGIN,
            max: egui::Pos2::new(objective.pos.x, objective.pos.y) + canvas_pos + (text_size / 2.0) + MARGIN,
        };
        // Fond du widget
        painter.rect_filled(
            rect_pos,
            egui::Rounding::same(3.0),
            self.get_objective_color(&objective.state),
        );

        // Bodures
        painter.rect_stroke(
            rect_pos,
            egui::Rounding::same(3.0),
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );

        // Text
        painter.text(
            egui::Pos2::new(objective.pos.x, objective.pos.y) + canvas_pos,
            egui::Align2::CENTER_CENTER,
            objective.title.to_string(),
            egui::FontId::proportional(20.0),
            colors::TEXT_COLOR,
        );
    }
}