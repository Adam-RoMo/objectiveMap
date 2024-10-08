use eframe::{egui::{self}, epaint::CubicBezierShape};
use crate::ui_components::{colors, CircleButton};

use objective_map_core::{objective::Vec2, Objective, ObjectiveState};

// const MAX_SIZE: egui::Vec2 = egui::Vec2 {
//     x: 200.0,
//     y: 100.0
// };

const MARGIN: egui::Vec2 = egui::Vec2 {
    x: 5.0,
    y: 3.0
};

#[derive(Clone)]
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

    fn get_text_size(&self, painter: &egui::Painter, objective: &Objective) -> egui::Vec2 {
        let text_size: egui::Vec2;

        text_size = match &objective.size {
            Some(n) => egui::Vec2 {x: n.x, y: n.y},
            None => painter.layout_no_wrap(objective.title.to_string(), egui::FontId::proportional(20.0), egui::Color32::WHITE).size()
        };
        text_size
    }

    fn get_rect_pos(&self, objective: &Objective, canvas_pos: egui::Vec2, text_size: egui::Vec2) -> egui::Rect {
        let pos_objective = match objective.pos {
            Some(pos) => pos,
            None => Vec2 {
                x: 0.0,
                y: 0.0
            }
        };
        let rect_pos = egui::Rect {
            min: egui::Pos2::new(pos_objective.x, pos_objective.y) + canvas_pos - (text_size / 2.0) - MARGIN,
            max: egui::Pos2::new(pos_objective.x, pos_objective.y) + canvas_pos + (text_size / 2.0) + MARGIN,
        };

        rect_pos
    }

    pub fn calculate_bezier_control_points(&self, start: egui::Pos2, end: egui::Pos2) -> (egui::Pos2, egui::Pos2) {
        let control1 = start + egui::Vec2::new(0.0, (end.y - start.y) / 2.0);
        let control2 = start + egui::Vec2::new(end.x - start.x, (end.y - start.y) / 2.0);

            
        (control1, control2)
    }

    pub fn draw_line_to_pos(&self, ui: &mut egui::Ui, canvas_pos: egui::Vec2, objective: &Objective, mouse_pos: egui::Pos2, top: bool) {
        let painter = ui.painter();
        let objective_text_size = self.get_text_size(painter, objective);
        let objective_rect_pos = self.get_rect_pos(objective, canvas_pos, objective_text_size);
        let pre_point: egui::Pos2;

        if top {
            pre_point = egui::Pos2 {
                x: objective_rect_pos.min.x + objective_rect_pos.size().x / 2.0,
                y: objective_rect_pos.min.y
            };
        } else {
            pre_point = egui::Pos2 {
                x: objective_rect_pos.min.x + objective_rect_pos.size().x / 2.0,
                y: objective_rect_pos.max.y
            };

        }
        let (mid_point1, mid_point2) = self.calculate_bezier_control_points(pre_point, mouse_pos);
        let points = [
            pre_point,
            mid_point1,
            mid_point2,
            mouse_pos
        ];

        let stroke = egui::Stroke::new(3.0, egui::Color32::from_black_alpha(200)); // Ajuste la couleur selon tes besoins
        let path = CubicBezierShape::from_points_stroke(points, false, egui::Color32::TRANSPARENT, stroke);
        painter.add(path);
    }

    pub fn draw_line<F>(&self, ui: &mut egui::Ui, canvas_pos: egui::Vec2, prerequisite: &Objective, edit_mode: bool, dependent: &Objective, on_click: F)
    where
        F: FnOnce(&Objective, &Objective),
    {
        let painter = ui.painter();
        let pre_text_size = self.get_text_size(painter, prerequisite);
        let dep_text_size = self.get_text_size(painter, dependent);
        let pre_rect_pos = self.get_rect_pos(prerequisite, canvas_pos, pre_text_size);
        let dep_rect_pos = self.get_rect_pos( dependent, canvas_pos, dep_text_size);

        let pre_point = egui::Pos2 {
            x: pre_rect_pos.min.x + pre_rect_pos.size().x / 2.0,
            y: pre_rect_pos.max.y
        };
        let dep_point = egui::Pos2 {
            x: dep_rect_pos.min.x + dep_rect_pos.size().x / 2.0,
            y: dep_rect_pos.min.y
        };
        let (mid_point1, mid_point2) = self.calculate_bezier_control_points(pre_point, dep_point);
        let points = [
            pre_point,
            mid_point1,
            mid_point2,
            dep_point
        ];

        let stroke = egui::Stroke::new(3.0, egui::Color32::from_black_alpha(200)); // Ajuste la couleur selon tes besoins
        let path = CubicBezierShape::from_points_stroke(points, false, egui::Color32::TRANSPARENT, stroke);
        painter.add(path);
        
        if edit_mode {
            let mut button = CircleButton::new(egui::Pos2::new((pre_point.x + dep_point.x) / 2.0, (pre_point.y + dep_point.y) / 2.0), 5.0, colors::ERROR_COLOR);
    
            button.ui(ui, || {
                on_click(prerequisite, dependent);
            });
        }

    }

    pub fn draw_edit_tools<F1, F2, F3>(&self, ui: &mut egui::Ui, canvas_pos: egui::Vec2, objective: &Objective, on_click1: F1, on_click2: F2, on_click3: F3)
    where
        F1: FnOnce(),
        F2: FnOnce(&Objective),
        F3: FnOnce(),
    {
        let text_size = self.get_text_size(ui.painter(), objective);
        let rect_pos = self.get_rect_pos(objective, canvas_pos, text_size);


        let mut top_middle_button = CircleButton::new(egui::Pos2::new(rect_pos.min.x + rect_pos.size().x / 2.0, rect_pos.min.y), 5.0, colors::ERROR_COLOR);
        let mut top_right_button = CircleButton::new(egui::Pos2::new(rect_pos.max.x, rect_pos.min.y), 5.0, colors::ERROR_COLOR);
        let mut bottom_middle_button = CircleButton::new(egui::Pos2::new(rect_pos.min.x + rect_pos.size().x / 2.0, rect_pos.max.y), 5.0, colors::ERROR_COLOR);

        top_middle_button.ui(ui, || {
            on_click1();
        });
        top_right_button.ui(ui, || {
            on_click2(objective);
        });
        bottom_middle_button.ui(ui, || {
            on_click3();
        });
    }

    pub fn draw_triangles(self, ui: &mut egui::Ui, canvas_pos: egui::Vec2, objective: &Objective) {
        let text_size = self.get_text_size(ui.painter(), objective);
        let rect_pos = self.get_rect_pos(objective, canvas_pos, text_size);
        let mid_pos = egui::Pos2::new(rect_pos.min.x + rect_pos.size().x / 2.0, rect_pos.max.y);

        let point1 = egui::Pos2::new(mid_pos.x - 5.0, mid_pos.y - 1.0);
        let point2 = egui::Pos2::new(mid_pos.x + 5.0, mid_pos.y - 1.0);
        let point3 = egui::Pos2::new(mid_pos.x, mid_pos.y + 10.0);

        let triangle = egui::Shape::convex_polygon(
            vec![point1, point2, point3],
            self.get_objective_color(&objective.state),
            egui::Stroke::NONE
        );

        // let triangle = egui::Shape::closed_line(vec![point1, point2, point3], egui::Stroke::new(1.0, ));
        ui.painter().add(triangle);
    }


    pub fn display(&self, ui: &mut egui::Ui, canvas_pos: egui::Vec2, objective: &Objective, selected: bool) ->  egui::Rect{
        let painter = ui.painter();
        let text_size = self.get_text_size(painter, objective);
        
        let rect_pos = self.get_rect_pos(objective, canvas_pos, text_size);
        let mut color = self.get_objective_color(&objective.state);
        // Fond du widget
        painter.rect_filled(
            rect_pos,
            egui::Rounding::same(3.0),
            color,
        );

        if selected {
            color = colors::TEXT2_COLOR;
        }
        // Bodures
        painter.rect_stroke(
            rect_pos,
            egui::Rounding::same(3.0),
            egui::Stroke::new(2.0, color),
        );
        let pos_objective = match objective.pos {
            Some(pos) => pos,
            None => Vec2 {
                x: 0.0,
                y: 0.0
            }
        };

        // Text
        painter.text(
            egui::Pos2::new(pos_objective.x, pos_objective.y) + canvas_pos,
            egui::Align2::CENTER_CENTER,
            objective.title.to_string(),
            egui::FontId::proportional(20.0),
            colors::TEXT_COLOR,
        );
        rect_pos
    }
}