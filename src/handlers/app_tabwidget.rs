use crate::data_structs::TaskListData;
use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::events::*;
use crate::ui_modules::TabItem;
use crate::ui_modules::TaskItem;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::Weak;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};


pub fn init(ui: &AppWindow, tx: Sender<(Arc<Mutex<Weak<AppWindow>>>, UIEvent)>, task_list_data: TaskListData) {
    // Convert task_list_data.lists into TabItem
    let tab_items: Vec<TabItem> = task_list_data.lists.into_iter().map(|list| {
        // Convert each Task into a TaskItem
        let task_items: Vec<TaskItem> = list.tasks.into_iter().map(|task| {
            TaskItem {
                // Convert Task fields into TaskItem fields
                title: task.title.clone().into(), // Adjust based on your actual struct fields
                checked: task.checked, // Example boolean field
            }
        }).collect();

        let task_items_model: Rc<VecModel<TaskItem>> = Rc::new(VecModel::from(task_items));
        let task_items_model_rc = ModelRc::from(task_items_model.clone());

        TabItem {
            title: list.title.into(), // Convert String to appropriate type
            items: task_items_model_rc,
        }
    }).collect();

    let items_model: Rc<VecModel<TabItem>> = Rc::new(VecModel::from(tab_items));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_tab_items(items_model_rc);
}