use petgraph::graph::NodeIndex;

pub enum ObjectiveState {
    Inaccessible,
    Pending,
    InProgress,
    Complete,
}

pub struct Vec2{
    pub x: f32,
    pub y: f32,
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
    pub task_list: Vec<String>,

    pub pos: Option<Vec2>,
    pub size: Option<Vec2>,
    pub node: NodeIndex
}

impl Objective {
    pub fn new(title: &str, description: &str, state: ObjectiveState, task_list: Vec<String>, pos: Option<Vec2>) -> Self {
        Objective {
            title: title.to_string(),
            description: description.to_string(),
            state,
            task_list,
            pos,
            size: None,
            node: NodeIndex::new(0)
        }
    }
}