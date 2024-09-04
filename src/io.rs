use crate::ui_modules::AppWindow;
use crate::ui_modules::TabItem;
use crate::ui_modules::TaskItem;
use serde_json::Result;
use slint::Model;
use slint::{ModelRc, VecModel, Weak};
use std::rc::Rc;

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::sync::{Arc, Mutex};
use toml;

use crate::data_structs::{Config, List, Task, TaskListData};

pub fn load_config() -> std::result::Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn load_data(config: &Config) -> Result<TaskListData> {
    // Open the data file specified in the config
    let file = File::open(&config.data_path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data into a Rust struct
    let data: TaskListData = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn save(
    app_state: Arc<Mutex<Weak<AppWindow>>>,
    config: &Config,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Get TaskListData from app state
    let weak_app_window = app_state.lock().unwrap();
    let app_window = weak_app_window
        .upgrade()
        .ok_or("Failed to upgrade Weak pointer to AppWindow")?;
    let task_list_data_rc = app_window.get_tab_items();
    let task_list_data = serialize_task_list_data(task_list_data_rc);

    // Serialize app data to JSON format
    let json = serde_json::to_string_pretty(&task_list_data)?;

    // Get access to the save file
    let mut file = File::create(&config.data_path)?;

    // Write JSON to the file specified by config
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn serialize_task_list_data(task_list_data_rc: ModelRc<TabItem>) -> TaskListData {
    let model = task_list_data_rc
        .as_any()
        .downcast_ref::<VecModel<TabItem>>()
        .unwrap();

    // Convert each TabItem in the ModelRc to a List
    let lists: Vec<List> = model
        .iter()
        .map(|tab_item| List {
            title: tab_item.title.to_string().clone(),
            tasks: tab_item
                .items
                .iter()
                .map(|task_item| Task {
                    title: task_item.title.to_string().clone(),
                    checked: task_item.checked,
                })
                .collect(),
        })
        .collect();

    TaskListData { lists }
}

pub fn deserialize_task_list_data(task_list_data: TaskListData) -> ModelRc<TabItem> {
    // Convert task_list_data.lists into TabItem
    let tab_items: Vec<TabItem> = task_list_data
        .lists
        .into_iter()
        .map(|list| {
            // Convert each Task into a TaskItem
            let task_items: Vec<TaskItem> = list
                .tasks
                .into_iter()
                .map(|task| {
                    TaskItem {
                        // Convert Task fields into TaskItem fields
                        title: task.title.clone().into(), // Adjust based on your actual struct fields
                        checked: task.checked,            // Example boolean field
                    }
                })
                .collect();

            let task_items_model: Rc<VecModel<TaskItem>> = Rc::new(VecModel::from(task_items));
            let task_items_model_rc = ModelRc::from(task_items_model.clone());

            TabItem {
                title: list.title.into(), // Convert String to appropriate type
                items: task_items_model_rc,
            }
        })
        .collect();

    let items_model: Rc<VecModel<TabItem>> = Rc::new(VecModel::from(tab_items));
    let items_model_rc = ModelRc::from(items_model.clone());

    items_model_rc
}
