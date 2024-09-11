use objective_map_core::{self, Guide, Objective, ObjectiveState};
use eframe::egui;


pub struct ObjectiveInfoWindow {
    pub modify_mode: bool,
    title: String,
    description: String
}

impl ObjectiveInfoWindow {
    pub fn new() -> Self {
        Self {
            modify_mode: false,
            title: String::new(),
            description: String::new()
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, guide: &mut Guide) {
        match guide.selected_objective {
            Some(node) => {
                // egui::Memory
                // let window_pos = ctx.memory(|mem| mem.window_pos("Window Title"));
                egui::Window::new(&guide.objectives[node].title)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label(&guide.objectives[node].description);
                    ui.group(|ui| {
                        for item in &guide.objectives[node].task_list {
                            ui.label(item);
                        }
                    });
                    if self.modify_mode == false {
                        if ui.button("Modifier").clicked() {
                            self.modify_mode = true;
                            self.title = guide.objectives[node].title.to_string();
                            self.description = guide.objectives[node].description.to_string();
                        }
                    }

                    if self.modify_mode == true {
                        ui.label("Title:");
                        ui.add(egui::TextEdit::singleline(&mut self.title));

                        ui.label("Description:");
                        ui.add(egui::TextEdit::singleline(&mut self.description));

                        // ui.label("Title:");
                        // ui.add(TextEdit::singleline(guide.objectives[node].title));

                        if ui.button("Enregistrer").clicked() {
                            self.modify_mode = false;
                            guide.objectives[node].title = self.title.to_string();
                            guide.objectives[node].description = self.description.to_string();
                        }
                    }

                    if guide.objectives[node].state == ObjectiveState::Pending {
                        if ui.button("Commencer").clicked() {
                            guide.objectives[node].state = ObjectiveState::InProgress;
                        }
                    }
                    if guide.objectives[node].state == ObjectiveState::InProgress {
                        if ui.button("Stopper").clicked() {
                            guide.objectives[node].state = ObjectiveState::Pending;
                        }    
                        if ui.button("Valider").clicked() {
                            guide.objectives[node].state = ObjectiveState::Complete;
                            guide.check_childs_status(node);
                        }
                    }    
                    
                });
            }
            None => ()
        }
    }
}