use crate::objective::{Objective, ObjectiveState, Variable};

use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use rfd::FileDialog;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SerializableNodeIndex(usize);

impl SerializableNodeIndex {
    // Méthode pour obtenir un NodeIndex à partir de SerializableNodeIndex
    pub fn to_node_index(&self) -> NodeIndex {
        NodeIndex::new(self.0)
    }
}

impl From<NodeIndex> for SerializableNodeIndex {
    fn from(index: NodeIndex) -> Self {
        SerializableNodeIndex(index.index())
    }
}

impl From<SerializableNodeIndex> for NodeIndex {
    fn from(sni: SerializableNodeIndex) -> Self {
        NodeIndex::new(sni.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectedObjectives {
    pub prerequisite: Option<SerializableNodeIndex>,
    pub dependent: Option<SerializableNodeIndex>
}

impl SelectedObjectives {
    pub fn is_full(&self) -> Option<(SerializableNodeIndex, SerializableNodeIndex)>{
        match self.prerequisite {
            None => None,
            Some(pre) => match self.dependent {
                None => None,
                Some(dep) => Some((pre, dep))
            }
        }
    }

    pub fn empty(&mut self) {
        self.prerequisite = None;
        self.dependent = None;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guide {
    pub title: String,
    pub description: String,
    pub objectives: DiGraph<Objective, String>,
    pub selected_objectives: SelectedObjectives,
    pub selected_objective: Option<SerializableNodeIndex>,
    pub variables: Vec<Variable>
}

impl Guide {
    pub fn new(title: &str, description: &str) -> Self {
        Guide {
            title: title.to_string(),
            description: description.to_string(),
            objectives: DiGraph::new(),
            selected_objectives: SelectedObjectives {
                prerequisite: None,
                dependent: None
            },
            selected_objective: None,
            variables: Vec::new()
        }
    }

    pub fn add_objective(&mut self, title: &str, description: &str, state: ObjectiveState) -> NodeIndex {
        let objective = Objective::new(
            title,
            description,
            state,
            Vec::new(),
            None,
        );
        let node = self.objectives.add_node(objective);
        self.objectives[node].node = SerializableNodeIndex::from(node);
        node
    }

    pub fn connect_objectives(&mut self, prerequisite: NodeIndex, dependent: NodeIndex, relation: &str) {
        self.objectives.add_edge(prerequisite, dependent, relation.to_string());
        self.check_childs_status(prerequisite);
    }

    pub fn remove_node(&mut self, node: NodeIndex) {
        match self.selected_objectives.dependent {
            Some(dep) => {
                if node == NodeIndex::from(dep) {
                    self.selected_objectives.dependent = None
                }
            }
            None => ()
        }
        match self.selected_objectives.prerequisite {
            Some(dep) => {
                if node == NodeIndex::from(dep) {
                    self.selected_objectives.prerequisite = None
                }
            }
            None => ()
        }
        self.objectives[node].state = ObjectiveState::Inaccessible;
        let neighbors: Vec<NodeIndex> = self.objectives
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .collect();
        self.objectives.remove_node(node);
        for neighbor in neighbors {
            self.check_childs_status(neighbor);
        }
    }

    pub fn remove_connection(&mut self, prerequisite: NodeIndex, dependent: NodeIndex) {
        if let Some(edge) = self.objectives.find_edge(prerequisite, dependent) {
            self.objectives.remove_edge(edge);
            self.check_childs_status(prerequisite);
            self.check_childs_status(dependent);
        }
    }

    pub fn check_childs_status(&mut self, node: NodeIndex) {
        let neighbors: Vec<NodeIndex> = self.objectives
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .collect();
        if neighbors.is_empty() && self.objectives[node].state != ObjectiveState::Complete
            && self.objectives[node].state != ObjectiveState::InProgress {
            self.objectives[node].state = ObjectiveState::Pending;
            return;
        }
        let neighbors: Vec<NodeIndex> = self.objectives
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .collect();
        for neighbor in neighbors {
            if self.objectives[neighbor].state == ObjectiveState::Pending &&
                (self.objectives[node].state == ObjectiveState::InProgress
                    || self.objectives[node].state == ObjectiveState::Inaccessible
                    || self.objectives[node].state == ObjectiveState::Pending
                ) {
                self.objectives[neighbor].state = ObjectiveState::Inaccessible;
            }
            if self.objectives[node].state == ObjectiveState::Complete
                && self.objectives[neighbor].state == ObjectiveState::Inaccessible {
                let mut accessible = true;
                let parents: Vec<NodeIndex> = self.objectives
                    .neighbors_directed(neighbor, petgraph::Direction::Incoming)
                    .collect();
                for parent in parents {
                    if self.objectives[parent].state != ObjectiveState::Complete {
                        accessible = false;
                    }
                }
                if accessible {
                    self.objectives[neighbor].state = ObjectiveState::Pending;
                } else {
                    self.objectives[neighbor].state = ObjectiveState::Inaccessible;
                }
            }
        }
    }

    pub fn auto_connect(&mut self) {
        match self.selected_objectives.is_full() {
            Some((prerequisite, dependent)) => {
                self.connect_objectives(NodeIndex::from(prerequisite), NodeIndex::from(dependent), "relation");
                self.selected_objectives.dependent = None;
                self.selected_objectives.prerequisite = None;
            },
            None => (),
        }
    }

    pub fn export_guide(guide: &Guide) {
        // Sérialiser le guide
        let serialized = match serde_json::to_string(guide) {
            Ok(serialized) => serialized,
            Err(err) => {
                eprintln!("Erreur de sérialisation : {}", err);
                return;
            }
        };
    
        // Sélectionner un fichier
        if let Some(file_path) = FileDialog::new()
            .set_title("Enregistrer le guide")
            .save_file()
            .map(|path| path.to_string_lossy().into_owned()) {
            
            // Créer le fichier
            let mut file = match File::create(&file_path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Erreur lors de la création du fichier : {}", err);
                    return;
                }
            };
            // Écrire dans le fichier
            if let Err(err) = file.write_all(serialized.as_bytes()) {
                eprintln!("Erreur lors de l'écriture dans le fichier : {}", err);
            }
        }
    }

    pub fn save_guide(guide: &Guide, mut file_path: &mut Option<String>) {
        if file_path.is_none() {
            if let Some(path) = FileDialog::new()
                .set_title("Enregistrer le guide")
                .save_file()
                .map(|p| p.to_string_lossy().into_owned()) {
                *file_path = Some(path);
            } else {
                return;
            }
        }

        if let Some(ref path) = *file_path {
            match File::create(path) {
                Ok(mut file) => {
                    match serde_json::to_string(guide) {
                        Ok(serialized) => {
                            if let Err(err) = file.write_all(serialized.as_bytes()) {
                                eprintln!("Erreur lors de l'écriture dans le fichier : {}", err);
                            }
                        },
                        Err(err) => {
                            eprintln!("Erreur de sérialisation : {}", err);
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Erreur lors de la création du fichier : {}", err);
                }
            }
        } else {
            eprintln!("Le chemin du fichier est introuvable.");
        }
    }

    pub fn load_guide() -> Option<Guide> {
        if let Some(path) = FileDialog::new()
            .set_title("Ouvrir un guide")
            .pick_file()
            .map(|p| p.to_string_lossy().into_owned()) {
            
            // Lire le fichier
            match File::open(&path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if let Err(err) = file.read_to_string(&mut contents) {
                        eprintln!("Erreur lors de la lecture du fichier : {}", err);
                        return None;
                    }
    
                    match serde_json::from_str(&contents) {
                        Ok(guide) => Some(guide),
                        Err(err) => {
                            eprintln!("Erreur de désérialisation : {}", err);
                            None
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Erreur lors de l'ouverture du fichier : {}", err);
                    None
                }
            }
        } else {
            None
        }
    }
}