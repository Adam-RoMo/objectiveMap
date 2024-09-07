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

pub struct Objective {
    pub title: String,
    pub description: String,
    pub state: ObjectiveState,
    pub task_list: Vec<String>,

    pub pos: Vec2,
    pub size: Option<Vec2>,
}

impl Objective {
    pub fn new(title: &str, description: &str, state: ObjectiveState, task_list: Vec<String>, pos: Vec2) -> Self {
        Objective {
            title: title.to_string(),
            description: description.to_string(),
            state,
            task_list,
            pos,
            size: None,
        }
    }
}