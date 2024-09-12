use eframe::egui::{self};
use objective_map_core::{Guide, Objective, ObjectiveState};


pub struct ObjectivesPanel {
    search_text: String,
    selected_status: Option<ObjectiveState>
}

impl ObjectivesPanel {
    pub fn new() -> Self {
        Self {
            search_text: String::new(),
            selected_status: None
        }
    }


    pub fn ui(&mut self, ctx: &egui::Context, guide: &mut Guide) {
        egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Recherche :");
                ui.add(egui::TextEdit::singleline(&mut self.search_text));
            });

            ui.horizontal(|ui| {
                ui.label("Filtrer par statut :");
                egui::ComboBox::from_label("Statut")
                    .selected_text(match self.selected_status {
                        None => "Tous les statuts".to_string(),
                        Some(ObjectiveState::Inaccessible) => "Inaccessible".to_string(),
                        Some(ObjectiveState::Pending) => "Accessible".to_string(),
                        Some(ObjectiveState::InProgress) => "En cours".to_string(),
                        Some(ObjectiveState::Complete) => "Complété".to_string(),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_status, None, "Tous les statuts");
                        ui.selectable_value(&mut self.selected_status, Some(ObjectiveState::Inaccessible), "Inaccessible");
                        ui.selectable_value(&mut self.selected_status, Some(ObjectiveState::Pending), "Accessible");
                        ui.selectable_value(&mut self.selected_status, Some(ObjectiveState::InProgress), "En cours");
                        ui.selectable_value(&mut self.selected_status, Some(ObjectiveState::Complete), "Complété");
                    });
            });

            ui.separator();
            for node in guide.objectives.node_indices() {
                if self.matches_search(&guide.objectives[node]) && self.matches_status(&guide.objectives[node]) {
                    ui.horizontal(|ui| {
                        ui.label(&guide.objectives[node].title);
                        if ui.button("Voir").clicked() {
                            guide.selected_objective = Some(node);
                            // println!("Objectif sélectionné : {}", guide.objectives[node].title);
                        }
                    });
                }
            }
        });

    }
    fn matches_search(&self, objective: &Objective) -> bool {
        self.search_text.is_empty() || objective.title.to_lowercase().contains(&self.search_text.to_lowercase())
    }

    fn matches_status(&self, objective: &Objective) -> bool {
        match &self.selected_status {
            Some(status) => objective.state == *status,
            None => true, // Si aucun statut n'est sélectionné, on affiche tout
        }
    }
}