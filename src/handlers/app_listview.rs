use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
// use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::ui_modules::TaskItem;
use slint::Model;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::ComponentHandle;
use std::rc::Rc;

pub enum AppListviewEvent {
    AddItem(SharedString),
    RemoveItem(),
    // Add more events here as needed
}

pub fn init(ui: &AppWindow) {
    // Initialize AppWindow items with placeholder values
    let items_model: Rc<VecModel<TaskItem>> =
        Rc::new(VecModel::from(vec![
            TaskItem {title: "Hello".into(), checked: false},
            TaskItem {title: "World".into(), checked: false}]
        ));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_items(items_model_rc);

    // Initialize add and remove handlers
    let ui_weak_1 = ui.as_weak();
    let ui_weak_2 = ui_weak_1.clone();
    ui.on_request_add_item(move || {
        let app = ui_weak_1.unwrap();
        let input_text = app.get_input_text();
        app.handle_event(AppListviewEvent::AddItem(input_text));
    });
    ui.on_request_remove_item(move || {
        let app = ui_weak_2.unwrap();
        app.handle_event(AppListviewEvent::RemoveItem());
    });
}

impl AppWindow {
    pub fn handle_event(&self, event: AppListviewEvent) {
        match event {
            AppListviewEvent::AddItem(text) => {
               handle_add_item(&self, text)
            },
            AppListviewEvent::RemoveItem() => {
                handle_remove_item(&self)
            }
            // Add more event handling here as needed
        }
    }
}

fn handle_add_item(app: &AppWindow, text: SharedString) {
    // Convert ModelRc to Model for access to Vector methods
    let items_model_rc = app.get_items();
    let items_model = items_model_rc
        .as_any()
        .downcast_ref::<VecModel<TaskItem>>()
        .expect("We know we set a VecModel earlier");

    // Appends user input and empties TextInput component
    if !text.is_empty() {
        items_model.push(TaskItem {title: text.into(), checked: false});
        app.set_input_text(SharedString::new());
    }
}

fn handle_remove_item(app: &AppWindow) {
    // Convert ModelRc to Model for access to Vector methods
    let items_model_rc = app.get_items();
    let items_model = items_model_rc
        .as_any()
        .downcast_ref::<VecModel<TaskItem>>()
        .expect("We know we set a VecModel earlier");

    // Removes checked items and adjusts offset to maintain index order
    let mut offset = 0;
    for i in 0..items_model.row_count() {
        if items_model.row_data(i - offset).unwrap().checked {
            items_model.remove(i - offset);
            offset += 1;
        }
    }
}
