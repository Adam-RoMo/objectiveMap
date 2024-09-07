use crate::objective::{self, Objective, ObjectiveState, Vec2};

use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

pub struct Guide {
    pub title: String,
    pub description: String,
    pub objectives: DiGraph<Objective, String>
}

impl Guide {
    pub fn new(title: &str, description: &str) -> Self {
        Guide {
            title: title.to_string(),
            description: description.to_string(),
            objectives: DiGraph::new(),
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
        self.objectives.add_node(objective)
    }

    pub fn connect_objectives(&mut self, prerequisite: NodeIndex, dependent: NodeIndex, relation: &str) {
        self.objectives.add_edge(prerequisite, dependent, relation.to_string());
    }
}