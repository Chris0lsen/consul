use crate::data_structs::TaskListData;
use crate::io;
use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::events::*;
use crate::ui_modules::TabItem;
use crate::ui_modules::TaskItem;
use slint::ComponentHandle;
use slint::Model;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::Weak;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn init(
    ui: &AppWindow,
    tx: Sender<(Arc<Mutex<Weak<AppWindow>>>, UIEvent)>,
    task_list_data: TaskListData,
) {
    let items_model_rc = io::deserialize_task_list_data(task_list_data);
    ui.set_tab_items(items_model_rc);

    // Initialize Arc to be cloned by each handler
    let ui_arc = Arc::new(Mutex::new(ui.as_weak()));

    let add_tab_handler_arc = Arc::clone(&ui_arc);
    let add_tab_tx = tx.clone();
    ui.on_request_add_tab(move || {
        // Clone Arc for thread
        let local_handler_clone = Arc::clone(&add_tab_handler_arc);

        let _ = add_tab_tx.send((
            local_handler_clone,
            UIEvent::AppTabWidget(AppTabWidgetEvent::AddTab()),
        ));
    });

    let remove_tab_handler_arc = Arc::clone(&ui_arc);
    let remove_tab_tx = tx.clone();
    ui.on_request_remove_tab(move || {
        // Clone Arc for thread
        let local_handler_clone = Arc::clone(&remove_tab_handler_arc);

        let _ = remove_tab_tx.send((
            local_handler_clone,
            UIEvent::AppTabWidget(AppTabWidgetEvent::RemoveTab()),
        ));
    });
}

pub fn handle_event(app_window: Arc<Mutex<Weak<AppWindow>>>, event: AppTabWidgetEvent) {
    match event {
        AppTabWidgetEvent::AddTab() => handle_add_tab(app_window),
        AppTabWidgetEvent::RemoveTab() => handle_remove_tab(app_window),
    }
}

fn handle_add_tab(app: Arc<Mutex<Weak<AppWindow>>>) {
    // Lock app
    let app_weak = app.lock().unwrap();

    let _ = app_weak.upgrade_in_event_loop(move |ui| {
        println!("WAKA FLOCKA FLAME");
        // Convert ModelRc to Model for access to Vector methods
        let tabs_model_rc = ui.get_tab_items();
        let tabs_model = tabs_model_rc
            .as_any()
            .downcast_ref::<VecModel<TabItem>>()
            .expect("We know we set a VecModel earlier");

        let text = ui.get_tab_input();

        // Appends user input and empties TextInput component
        if !text.is_empty() {
            tabs_model.push(TabItem {
                title: text.into(),
                items: ui.get_items(),
            });
            ui.set_tab_input(SharedString::new());
        }
    });
}

fn handle_remove_tab(app: Arc<Mutex<Weak<AppWindow>>>) {
    // Lock app
    let app_weak = app.lock().unwrap();

    let _ = app_weak.upgrade_in_event_loop(move |ui| {
        // Convert ModelRc to Model for access to Vector methods
        let tabs_model_rc = ui.get_tab_items();
        let tabs_model = tabs_model_rc
            .as_any()
            .downcast_ref::<VecModel<TabItem>>()
            .expect("We know we set a VecModel earlier");
        // Remove currently active tab
        let active_tab = ui.get_active_tab().try_into().unwrap();
        tabs_model.remove(active_tab);
    });
}

#[cfg(test)]
mod tests {
    use crate::data_structs::{List, Task};

    use super::*;
    use std::sync::mpsc;

    fn setup() -> AppWindow {
        let ui = AppWindow::new().unwrap();
        type Message = (Arc<Mutex<Weak<AppWindow>>>, UIEvent);
        let (tx, _): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
        let tld = TaskListData {
            lists: [List {
                title: "Home".to_string(),
                tasks: [Task {
                    title: "Hello".to_string(),
                    checked: false,
                }]
                .to_vec(),
            }]
            .to_vec(),
        };
        init(&ui, tx, tld);

        let item_model = Rc::new(VecModel::from(vec![TaskItem {
            title: "Hello".into(),
            checked: false,
        }]));
        let item_model_rc = ModelRc::from(item_model.clone());
        ui.set_items(item_model_rc);

        ui
    }

    #[test]
    fn test_handle_add_tab() {
        let _ = slint::invoke_from_event_loop(|| {
            let ui = setup();
            ui.set_tab_input(SharedString::from("Goodbye"));
            let ui_arc = Arc::new(Mutex::new(ui.as_weak()));
            let new_tab_model_rc = ui.get_tab_items();
            let new_tab_model = new_tab_model_rc
                .as_any()
                .downcast_ref::<VecModel<TabItem>>()
                .expect("We know we set a VecModel earlier");

            handle_add_tab(ui_arc);

            assert_eq!(new_tab_model.row_data(2).unwrap().title, "Goodbye");
        });
    }

    #[test]
    fn test_handle_remove_tab() {
        let _ = slint::invoke_from_event_loop(|| {
            let ui = setup();
            ui.set_active_tab(1);
            let ui_arc = Arc::new(Mutex::new(ui.as_weak()));
            let new_tab_model_rc = ui.get_tab_items();
            let new_tab_model = new_tab_model_rc
                .as_any()
                .downcast_ref::<VecModel<TabItem>>()
                .expect("We know we set a VecModel earlier");

            handle_remove_tab(ui_arc);

            assert_eq!(new_tab_model.row_count(), 1);
        });
    }
}
