use eframe::egui;
use crate::ui_components::colors;
use crate::ui_components::objective_widget::ObjectiveWidget;

use objective_map_core::{Objective, ObjectiveState};

pub struct MovableCanvas {
    canvas_pos: egui::Vec2,
    canvas_items: Vec<ObjectiveWidget>,
    dragging: Option<egui::Pos2>,
}

impl MovableCanvas {
    pub fn new() -> Self {
        Self {
            canvas_pos: egui::Vec2::ZERO,
            canvas_items: vec![
                ObjectiveWidget::new(
                    Objective::new("This objective is longgggg", "The description", ObjectiveState::Inaccessible, Vec::new()),
                    egui::Pos2::new(50.0, 50.0),
                    // egui::Rect::from_min_size(egui::Pos2::new(50.0, 50.0), egui::Vec2::new(100.0, 100.0))
                )
            ],
            dragging: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // let available_rect = ui.available_rect_before_wrap();
        let canvas_rect = egui::Rect::EVERYTHING;
        let response = ui.allocate_rect(canvas_rect, egui::Sense::drag());

        if response.dragged() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                if let Some(prev_drag_pos) = self.dragging {
                    self.canvas_pos += mouse_pos - prev_drag_pos;
                }
                self.dragging = Some(mouse_pos);
            }
        } else {
            self.dragging = None;
        }
        dbg!(self.dragging);
        // Dessiner le canvas
        let painter = ui.painter();
        painter.rect_filled(canvas_rect, 0.0, colors::BACKGROUND2_COLOR);

        // Dessiner les éléments sur le canvas
        for item in &mut self.canvas_items {
            item.display(painter, self.canvas_pos);
        }
    }
}