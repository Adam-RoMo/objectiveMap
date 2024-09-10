use crate::objective::{Objective, ObjectiveState, Vec2};

use petgraph::adj::Neighbors;
use petgraph::graph::DiGraph;
use petgraph::graph::Node;
use petgraph::graph::NodeIndex;



pub struct SelectedObjectives {
    pub prerequisite: Option<NodeIndex>,
    pub dependent: Option<NodeIndex>
}

impl SelectedObjectives {
    pub fn is_full(&self) -> Option<(NodeIndex, NodeIndex)>{
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
pub struct Guide {
    pub title: String,
    pub description: String,
    pub objectives: DiGraph<Objective, String>,
    pub selected_objectives: SelectedObjectives,
    pub selected_objective: Option<NodeIndex>
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
            selected_objective: None
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
        self.objectives[node].node = node;
        node
    }

    pub fn connect_objectives(&mut self, prerequisite: NodeIndex, dependent: NodeIndex, relation: &str) {
        self.objectives.add_edge(prerequisite, dependent, relation.to_string());
        self.check_childs_status(prerequisite);
    }

    pub fn remove_node(&mut self, node: NodeIndex) {
        match self.selected_objectives.dependent {
            Some(dep) => {
                if node == dep {
                    self.selected_objectives.dependent = None
                }
            }
            None => ()
        }
        match self.selected_objectives.prerequisite {
            Some(dep) => {
                if node == dep {
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
                self.objectives[neighbor].state = ObjectiveState::Pending;
            }
        }
    }

    pub fn auto_connect(&mut self) {
        match self.selected_objectives.is_full() {
            Some((prerequisite, dependent)) => {
                self.connect_objectives(prerequisite, dependent, "relation");
                self.selected_objectives.dependent = None;
                self.selected_objectives.prerequisite = None;
            },
            None => (),
        }
    }
}