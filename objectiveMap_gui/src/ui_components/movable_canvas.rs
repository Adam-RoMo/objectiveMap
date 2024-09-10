use eframe::egui;
use crate::ui_components::colors;
use crate::ui_components::objective_widget::ObjectiveWidget;

use objective_map_core::{self, Guide, Objective, ObjectiveState};

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

    pub fn ui(&mut self, ui: &mut egui::Ui, guide: &mut Guide, edit_mode: bool) {
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
        let node_indices: Vec<_> = guide.objectives.node_indices().collect();

        // Dessiner les éléments sur le canvas
        for node in node_indices {
            objective_widget.display(ui, self.canvas_pos, &guide.objectives[node]);
        }


        let mut edges_to_remove = Vec::new();

        // Dessiner les edges
        for edge in guide.objectives.edge_indices() {
            let (prerequisite, dependent) = guide.objectives.edge_endpoints(edge).unwrap();

            objective_widget.draw_line(ui, self.canvas_pos, &guide.objectives[prerequisite], &guide.objectives[dependent],
            |prerequisite, dependent| {
                edges_to_remove.push((prerequisite.node, dependent.node));
            });
        }

        for (prerequisite, dependent) in edges_to_remove {
            guide.remove_connection(prerequisite, dependent);
        }

        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
            match guide.selected_objectives.dependent {
                Some(node) => objective_widget.draw_line_to_pos(ui, self.canvas_pos, &guide.objectives[node], mouse_pos, true),
                None => ()
            }
            match guide.selected_objectives.prerequisite {
                Some(node) => objective_widget.draw_line_to_pos(ui, self.canvas_pos, &guide.objectives[node], mouse_pos, false),
                None => ()
            }
        }

        let node_indices: Vec<_> = guide.objectives.node_indices().collect();
        let mut nodes_to_remove = Vec::new();

        // edit mode
        if edit_mode {
            for node in node_indices {
                objective_widget.draw_edit_tools(ui, self.canvas_pos, &mut guide.objectives[node],
                    || {
                        guide.selected_objectives.dependent = Some(node);
                    },
                    |objective| {
                        nodes_to_remove.push(objective.node);
                        // guide.remove_node(objective.node);
                    },
                    || {
                        guide.selected_objectives.prerequisite = Some(node);
                    }
                );
            }
            guide.auto_connect();
            for node in nodes_to_remove {
                guide.remove_node(node);
            }
        }

    }
}