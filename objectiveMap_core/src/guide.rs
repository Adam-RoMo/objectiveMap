use std::ptr::null;

use crate::objective::{self, Objective, ObjectiveState, Vec2};

use petgraph::graph::DiGraph;
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
    pub selected_objectives: SelectedObjectives
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
            }
        }
    }

    pub fn add_objective(&mut self, title: &str, description: &str, state: ObjectiveState, position: Vec2) -> NodeIndex {
        let objective = Objective::new(
            title,
            description,
            state,
            Vec::new(),
            position
        );
        let node = self.objectives.add_node(objective);
        self.objectives[node].node = node;
        node
    }

    pub fn connect_objectives(&mut self, prerequisite: NodeIndex, dependent: NodeIndex, relation: &str) {
        self.objectives.add_edge(prerequisite, dependent, relation.to_string());
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