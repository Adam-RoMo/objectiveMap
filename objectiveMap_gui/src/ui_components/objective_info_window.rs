use objective_map_core::{self, Guide, ObjectiveState, Variable};
use eframe::egui;

pub struct ObjectiveInfoWindow {
    pub modify_mode: bool,
    pub add_mode: bool,
    title: String,
    description: String
}

impl ObjectiveInfoWindow {
    pub fn new() -> Self {
        Self {
            modify_mode: false,
            add_mode: false,
            title: String::new(),
            description: String::new()
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, guide: &mut Guide) {
        match guide.selected_objective {
            Some(node) => {
                egui::Window::new(&guide.objectives[node.to_node_index()].title)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(&guide.objectives[node.to_node_index()].description);
                    if self.modify_mode == false {
                        ui.group(|ui| {
                            ui.label("Prérequis text:");
                            for item in &mut guide.objectives[node.to_node_index()].task_list {
                                ui.checkbox(&mut item.1, item.0.to_string());
                            }
                        });

                        ui.separator();
                        ui.group(|ui| {
                            ui.label("Prérequis variable:");
                            for variable in &mut guide.objectives[node.to_node_index()].variable_requirements {
                                let mut bool = false;
    
                                if let Some(index) = guide.variables.iter().position(|x| x.name == variable.name) {
                                    bool = guide.variables[index].value >= variable.value;
                                    ui.add_enabled(false, |ui: &mut egui::Ui| {
                                        ui.checkbox(&mut bool, variable.name.to_string())
                                    });
                                }
                            }
                        });
                    }

                    if self.modify_mode == true {
                        ui.label("Title:");
                        ui.add(egui::TextEdit::singleline(&mut self.title));

                        ui.label("Description:");
                        ui.add(egui::TextEdit::singleline(&mut self.description));
    
                        ui.group(|ui| {
                            ui.label("Prérequis text:");
                            for item in &mut guide.objectives[node.to_node_index()].task_list {
                                    ui.add(egui::TextEdit::singleline(&mut item.0));
                            }
                            if ui.button("Nouveau").clicked() {
                                guide.objectives[node.to_node_index()].task_list.push(("Nouveau prérequis".to_string(), false));
                                self.add_mode = false;

                            }
                            ui.separator();

                            ui.label("Prérequis variable:");
                            for variable in &mut guide.objectives[node.to_node_index()].variable_requirements {
                                ui.horizontal(|ui| {
                                    ui.label(variable.name.to_string());
                                    ui.add(egui::DragValue::new(&mut variable.value).speed(1));
                                });
                            }
                            egui::ComboBox::from_id_source("var_choice").selected_text("Nouveau").show_ui(ui, |ui| {
                                for variable in &mut guide.variables {
                                    if ui.selectable_label(false, &variable.name).clicked() {
                                        guide.objectives[node.to_node_index()].variable_requirements.push(
                                            Variable {
                                                name: variable.name.to_string(),
                                                value: 0
                                            }
                                        );
                                        self.add_mode = false;
                                    }
                                }
                            });
                        });

                        if ui.button("Enregistrer").clicked() {
                            self.modify_mode = false;
                            self.add_mode = false;
                            guide.objectives[node.to_node_index()].title = self.title.to_string();
                            guide.objectives[node.to_node_index()].description = self.description.to_string();
                        }
                    }
                    ui.horizontal(|ui| {
                        if self.modify_mode == false {
                            if ui.button("Modifier").clicked() {
                                self.modify_mode = true;
                                self.title = guide.objectives[node.to_node_index()].title.to_string();
                                self.description = guide.objectives[node.to_node_index()].description.to_string();
                            }
                            if guide.objectives[node.to_node_index()].state == ObjectiveState::Pending {
                                if ui.button("Commencer").clicked() {
                                    guide.objectives[node.to_node_index()].state = ObjectiveState::InProgress;
                                }
                            }
                            if guide.objectives[node.to_node_index()].state == ObjectiveState::InProgress {
                                if ui.button("Stopper").clicked() {
                                    guide.objectives[node.to_node_index()].state = ObjectiveState::Pending;
                                }    
                                if ui.button("Valider").clicked() {
                                    guide.objectives[node.to_node_index()].state = ObjectiveState::Complete;
                                    guide.check_childs_status(node.to_node_index());
                                }
                            }    
                        }
                    });
                });
            }
            None => ()
        }
    }
}