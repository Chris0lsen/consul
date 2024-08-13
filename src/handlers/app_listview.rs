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
    let items_model: Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_items(items_model_rc);

    let task_items_model: Rc<VecModel<TaskItem>> =
        Rc::new(VecModel::from(vec![
            TaskItem {title: "Hello".into(), checked: false},
            TaskItem {title: "World".into(), checked: false}]
        ));
    let task_items_model_rc = ModelRc::from(task_items_model.clone());
    ui.set_task_items(task_items_model_rc);

    let ui_weak = ui.as_weak();
    ui.on_request_add_item(move || {
        let app = ui_weak.unwrap();
        let input_text = app.get_input_text();
        app.handle_event(AppListviewEvent::AddItem(input_text));
    });

    let ui_weak = ui.as_weak();
    ui.on_request_remove_item(move || {
        let app = ui_weak.unwrap();
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
    let items_model_rc = app.get_items();
    let items_model = items_model_rc
        .as_any()
        .downcast_ref::<VecModel<SharedString>>()
        .expect("We know we set a VecModel earlier");
    let task_items_model_rc = app.get_task_items();
    let task_items_model = task_items_model_rc
        .as_any()
        .downcast_ref::<VecModel<TaskItem>>()
        .expect("We know we set a VecModel earlier");
    if !text.is_empty() {
        let task_text = text.clone();
        items_model.push(SharedString::from(text));
        task_items_model.push(TaskItem {title: task_text.into(), checked: false});
        app.set_input_text(SharedString::new());
    }
}

fn handle_remove_item(app: &AppWindow) {
    let items_model_rc = app.get_items();
    let items_model = items_model_rc
        .as_any()
        .downcast_ref::<VecModel<SharedString>>()
        .expect("We know we set a VecModel earlier");

    let task_items_model_rc = app.get_task_items();
    let task_items_model = task_items_model_rc
        .as_any()
        .downcast_ref::<VecModel<TaskItem>>()
        .expect("We know we set a VecModel earlier");

    let mut offset = 0;
    for i in 0..task_items_model.row_count() {
        if task_items_model.row_data(i - offset).unwrap().checked {
            task_items_model.remove(i - offset);
            items_model.remove(i - offset);
            offset += 1;
        }
    }
}
