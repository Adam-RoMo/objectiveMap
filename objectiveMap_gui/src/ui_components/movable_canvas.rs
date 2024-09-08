use eframe::egui;
use crate::ui_components::colors;
use crate::ui_components::objective_widget::ObjectiveWidget;

use objective_map_core::{Guide, Objective, ObjectiveState};

pub struct MovableCanvas {
    canvas_pos: egui::Vec2,
    // canvas_items: Vec<ObjectiveWidget>,
    dragging: Option<egui::Pos2>,
}

impl MovableCanvas {
    pub fn new() -> Self {
        Self {
            canvas_pos: egui::Vec2::ZERO,
            dragging: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, guide: &mut Guide) {
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
        // Dessiner le canvas
        let painter = ui.painter();
        painter.rect_filled(canvas_rect, 0.0, colors::BACKGROUND2_COLOR);

        let objective_widget: ObjectiveWidget = ObjectiveWidget::new();
        // Dessiner les éléments sur le canvas
        for node in guide.objectives.node_indices() {
            objective_widget.display(painter, self.canvas_pos, &guide.objectives[node])
        }

        // Dessiner les edges
        for edge in guide.objectives.edge_indices() {
            let (prerequisite, dependent) = guide.objectives.edge_endpoints(edge).unwrap();

            objective_widget.draw_line(painter, self.canvas_pos, &guide.objectives[prerequisite], &guide.objectives[dependent]);
        }
    }
}