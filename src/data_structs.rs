use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub checked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub title: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskListData {
    pub lists: Vec<List>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_path: String,
}
