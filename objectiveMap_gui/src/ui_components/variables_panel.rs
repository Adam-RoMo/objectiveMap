use eframe::egui::{self};
use objective_map_core::{Guide, Variable};

pub struct VariablesPanel {}

impl VariablesPanel {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, guide: &mut Guide) {
        egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            for variable in &mut guide.variables {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut variable.name));
                    ui.add(egui::DragValue::new(&mut variable.value).speed(1));
                });
            }
            if ui.button("Ajouter variable").clicked() {
                guide.variables.push(Variable::new());
            }
        });
    }
}