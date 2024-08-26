use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::ui_modules::TaskItem;
use crate::UIEvent;
use slint::ComponentHandle;
use slint::Model;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::Weak;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn init(ui: &AppWindow, tx: Sender<(Arc<Mutex<Weak<AppWindow>>>, UIEvent)>) {
    // Initialize AppWindow items with placeholder values
    let items_model: Rc<VecModel<TaskItem>> = Rc::new(VecModel::from(vec![
        TaskItem {
            title: "Hello".into(),
            checked: false,
        },
        TaskItem {
            title: "World".into(),
            checked: false,
        },
    ]));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_items(items_model_rc);

    // Initialize Arc to be cloned by each handler
    let ui_arc = Arc::new(Mutex::new(ui.as_weak()));

    let add_item_handler_arc = Arc::clone(&ui_arc);
    let add_item_tx = tx.clone();
    ui.on_request_add_item(move || {
        // Clone Arc for thread
        let local_handler_clone = Arc::clone(&add_item_handler_arc);

        let _ = add_item_tx.send((local_handler_clone, UIEvent::AddItem()));
    });

    let remove_item_handler_arc = Arc::clone(&ui_arc);
    let remove_item_tx = tx.clone();
    ui.on_request_remove_checked_items(move || {
        // Clone Arc for thread
        let local_handler_clone = Arc::clone(&remove_item_handler_arc);

        let _ = remove_item_tx.send((local_handler_clone, UIEvent::RemoveCheckedItems()));
    });

    let save_item_handler_arc = Arc::clone(&ui_arc);
    let save_item_tx = tx.clone();
    ui.on_request_save_item(move |index, update| {
        let local_handler_clone = Arc::clone(&save_item_handler_arc);

        let _ = save_item_tx.send((
            local_handler_clone,
            UIEvent::SaveItem(index.try_into().unwrap(), update),
        ));
    })
}

pub fn handle_add_item(app: Arc<Mutex<Weak<AppWindow>>>) {
    // Lock app
    let app_weak = app.lock().unwrap();

    let _ = app_weak.upgrade_in_event_loop(move |ui| {
        // Convert ModelRc to Model for access to Vector methods
        let items_model_rc = ui.get_items();
        let items_model = items_model_rc
            .as_any()
            .downcast_ref::<VecModel<TaskItem>>()
            .expect("We know we set a VecModel earlier");

        let text = ui.get_input_text();

        // Appends user input and empties TextInput component
        if !text.is_empty() {
            items_model.push(TaskItem {
                title: text.into(),
                checked: false,
            });
            ui.set_input_text(SharedString::new());
        }
    });
}

pub fn handle_remove_checked_items(app: Arc<Mutex<Weak<AppWindow>>>) {
    // Lock app
    let app_weak = app.lock().unwrap();

    let _ = app_weak.upgrade_in_event_loop(move |ui| {
        // Convert ModelRc to Model for access to Vector methods
        let items_model_rc = ui.get_items();
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
    });
}

pub fn handle_save_item(app: Arc<Mutex<Weak<AppWindow>>>, index: usize, update: SharedString) {
    let app_weak = app.lock().unwrap();

    let _ = app_weak.upgrade_in_event_loop(move |ui| {
        // Convert ModelRc to Model for access to Vector methods
        let items_model_rc = ui.get_items();
        let items_model = items_model_rc
            .as_any()
            .downcast_ref::<VecModel<TaskItem>>()
            .expect("We know we set a VecModel earlier");

        let new_task_item = TaskItem {
            title: update,
            checked: items_model.row_data(index).unwrap().checked,
        };
        items_model.set_row_data(index, new_task_item);
    });
}
