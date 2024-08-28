use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    title: String,
    checked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    title: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskListData {
    lists: Vec<List>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data_path: String,
}
