pub enum ObjectiveState {
    Inaccessible,
    Pending,
    InProgress,
    Complete,
}

pub struct Objective {
    pub title: String,
    pub description: String,
    pub state: ObjectiveState,
    pub task_list: Vec<String> 
}

impl Objective {
    pub fn new(title: &str, description: &str, state: ObjectiveState, task_list: Vec<String>) -> Self {
        Objective {
            title: title.to_string(),
            description: description.to_string(),
            state,
            task_list
        }
    }
}