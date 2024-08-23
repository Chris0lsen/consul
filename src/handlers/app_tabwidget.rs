use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::ui_modules::TaskItem;
use crate::ui_modules::TabItem;
// use slint::ComponentHandle;
// use slint::Model;
use slint::ModelRc;
// use slint::SharedString;
use slint::VecModel;
use std::rc::Rc;
// use std::sync::{Arc, Mutex};
// use std::thread;

// pub enum AppTabWidgetEvent {
//     AddItem(SharedString),
//     RemoveItem(),
//     // Add more events here as needed
// }

pub fn init(ui: &AppWindow) {
    let tab_model = Rc::new(VecModel::from(vec![
        TaskItem {
            title: "Hello".into(),
            checked: false,
        }
    ]));
    let tab_model_rc = ModelRc::from(tab_model.clone());
    let tmr_2 = tab_model_rc.clone();

    // Initialize AppWindow items with placeholder values
    let items_model: Rc<VecModel<TabItem>> = Rc::new(VecModel::from(vec![
        TabItem {
            title: "Hello".into(),
            items: tab_model_rc,
        },
        TabItem {
            title: "World".into(),
            items: tmr_2,
        },
    ]));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_tab_items(items_model_rc);

    // Initialize Arc to be cloned by each handler
    // let ui_arc = Arc::new(Mutex::new(ui.as_weak()));

    // let add_item_handler_arc = Arc::clone(&ui_arc);
    // ui.on_request_add_item(move || {
    //     // Clone Arc for thread
    //     let local_handler_clone = Arc::clone(&add_item_handler_arc);
    //     thread::spawn(move || {
    //         // Locks Weak UI reference until it goes out of scope
    //         let ui_weak = local_handler_clone.lock().unwrap();
    //         // Capture unused Result in placeholder variable
    //         let _ = ui_weak.upgrade_in_event_loop(move |ui| {
    //             let text = ui.get_input_text();
    //             ui.handle_tab_event(AppTabWidgetEvent::AddItem(text));
    //         });
    //     });
    // });

    // let remove_item_handler_arc = Arc::clone(&ui_arc);
    // ui.on_request_remove_item(move || {
    //     // Clone Arc for thread
    //     let local_handler_clone = Arc::clone(&remove_item_handler_arc);
    //     thread::spawn(move || {
    //         // Locks Weak UI reference until it goes out of scope
    //         let ui_weak = local_handler_clone.lock().unwrap();
    //         // Capture unused Result in placeholder variable
    //         let _ = ui_weak.upgrade_in_event_loop(move |ui| {
    //             ui.handle_tab_event(AppTabWidgetEvent::RemoveItem());
    //         });
    //     });
    // });
}

// impl AppWindow {
//     pub fn handle_tab_event(&self, event: AppTabWidgetEvent) {
//         match event {
//             AppTabWidgetEvent::AddItem(text) => handle_add_item(&self, text),
//             AppTabWidgetEvent::RemoveItem() => handle_remove_item(&self), // Add more event handling here as needed
//         }
//     }
// }

// fn handle_add_item(app: &AppWindow, text: SharedString) {
//     // Convert ModelRc to Model for access to Vector methods
//     let items_model_rc = app.get_items();
//     let items_model = items_model_rc
//         .as_any()
//         .downcast_ref::<VecModel<TaskItem>>()
//         .expect("We know we set a VecModel earlier");

//     // Appends user input and empties TextInput component
//     if !text.is_empty() {
//         items_model.push(TaskItem {
//             title: text.into(),
//             checked: false,
//         });
//         app.set_input_text(SharedString::new());
//     }
// }

// fn handle_remove_item(app: &AppWindow) {
//     // Convert ModelRc to Model for access to Vector methods
//     let items_model_rc = app.get_items();
//     let items_model = items_model_rc
//         .as_any()
//         .downcast_ref::<VecModel<TaskItem>>()
//         .expect("We know we set a VecModel earlier");

//     // Removes checked items and adjusts offset to maintain index order
//     let mut offset = 0;
//     for i in 0..items_model.row_count() {
//         if items_model.row_data(i - offset).unwrap().checked {
//             items_model.remove(i - offset);
//             offset += 1;
//         }
//     }
// }
