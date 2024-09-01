use eframe::egui::{self};
use crate::ui_components::colors;

pub struct CanvasItem {
    pub rect: egui::Rect,
    pub color: egui::Color32,
}

pub struct MovableCanvas {
    canvas_pos: egui::Vec2,
    canvas_items: Vec<CanvasItem>,
    dragging: Option<egui::Pos2>,
}

impl MovableCanvas {
    pub fn new() -> Self {
        Self {
            canvas_pos: egui::Vec2::ZERO,
            canvas_items: vec![
                CanvasItem {
                    rect: egui::Rect::from_min_size(egui::Pos2::new(50.0, 50.0), egui::Vec2::new(100.0, 100.0)),
                    color: egui::Color32::from_rgb(200, 100, 100),
                },
                CanvasItem {
                    rect: egui::Rect::from_min_size(egui::Pos2::new(200.0, 150.0), egui::Vec2::new(150.0, 150.0)),
                    color: egui::Color32::from_rgb(100, 200, 100),
                },
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

        // Dessiner le canvas
        let painter = ui.painter();
        painter.rect_filled(canvas_rect, 0.0, colors::BACKGROUND2_COLOR);

        // Dessiner les éléments sur le canvas
        for item in &self.canvas_items {
            let item_rect = item.rect.translate(self.canvas_pos);
            painter.rect_filled(item_rect, 0.0, item.color);
        }
    }
}