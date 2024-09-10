use eframe::egui;
use crate::ui_components::colors;
use crate::ui_components::objective_widget::ObjectiveWidget;

use objective_map_core::{self, objective::Vec2, Guide, Objective, ObjectiveState};

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
        let canvas_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(canvas_rect, egui::Sense::drag());
        
        // Déplacemer la carte
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
            if guide.objectives[node].pos.is_none() {
                let canvas_size = canvas_rect.size();
                guide.objectives[node].pos = Some(Vec2{
                    x: canvas_size.x / 2.0 - self.canvas_pos.x,
                    y: canvas_size.y / 2.0 - self.canvas_pos.y
                })
            }
            let rect = objective_widget.display(ui, self.canvas_pos, &guide.objectives[node]);
            let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            // let response_click = ui.allocate_rect(rect, egui::Sense::click_and_drag());

            // Si l'objectif est en train d'être déplacé
            if response.dragged() {
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    guide.objectives[node].pos = Some(Vec2 {
                        x: mouse_pos.x - self.canvas_pos.x,
                        y: mouse_pos.y - self.canvas_pos.y
                    });
                }
            }

            // Si l'objectif est clické
            if response.clicked() {
                guide.selected_objective = Some(node);
            }
        }

        // Dessiner les edges
        let mut edges_to_remove = Vec::new();

        for edge in guide.objectives.edge_indices() {
            let (prerequisite, dependent) = guide.objectives.edge_endpoints(edge).unwrap();

            objective_widget.draw_line(ui, self.canvas_pos, &guide.objectives[prerequisite], &guide.objectives[dependent],
            |prerequisite, dependent| {
                edges_to_remove.push((prerequisite.node, dependent.node));
            });
        }

        // Remove connection
        for (prerequisite, dependent) in edges_to_remove {
            guide.remove_connection(prerequisite, dependent);
        }

        // Draw line to mouse
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

        // edit mode
        let node_indices: Vec<_> = guide.objectives.node_indices().collect();
        let mut nodes_to_remove = Vec::new();

        if edit_mode {
            for node in node_indices {
                objective_widget.draw_edit_tools(ui, self.canvas_pos, &mut guide.objectives[node],
                    || {
                        guide.selected_objectives.dependent = Some(node);
                    },
                    |objective| {
                        nodes_to_remove.push(objective.node);
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