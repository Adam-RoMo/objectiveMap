use petgraph::graph::NodeIndex;

#[derive(Debug, PartialEq)]
pub enum ObjectiveState {
    Inaccessible,
    Pending,
    InProgress,
    Complete,
}

pub struct Variable {
    pub name: String,
    pub value: u32
}

impl Variable {
    pub fn new() -> Self {
        Variable {
            name: "New Variable".to_string(),
            value: 0
        }
    }
}

pub struct Vec2{
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Vec2 {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Copy for Vec2 {}

impl Clone for Vec2 {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Objective {
    pub title: String,
    pub description: String,
    pub state: ObjectiveState,
    pub task_list: Vec<(String, bool)>,
    pub variable_requirements: Vec<Variable>, 

    pub pos: Option<Vec2>,
    pub size: Option<Vec2>,
    pub node: NodeIndex,
}

impl Objective {
    pub fn new(title: &str, description: &str, state: ObjectiveState, variable_requirements: Vec<Variable>, pos: Option<Vec2>) -> Self {
        Objective {
            title: title.to_string(),
            description: description.to_string(),
            state,
            task_list: Vec::new(),
            variable_requirements,
            pos,
            size: None,
            node: NodeIndex::new(0)
        }
    }
}