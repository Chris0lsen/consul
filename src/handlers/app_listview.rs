use crate::ui::AppWindow; // Import the re-exported AppWindow type
// use crate::ui_events::UIEvent; // Import the UIEvent enum
use slint::Model;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use slint::ComponentHandle;
use std::rc::Rc;

pub enum AppListviewEvent {
    AddItem(SharedString),
    // Add more events here as needed
}

pub fn init(ui: &AppWindow) {
    let items_model: Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
    let items_model_rc = ModelRc::from(items_model.clone());
    ui.set_items(items_model_rc);
    let ui_weak = ui.as_weak();
    ui.on_request_add_item(move || {
        let app = ui_weak.unwrap();
        let input_text = app.get_input_text();
        app.handle_event(AppListviewEvent::AddItem(input_text));
    });
}

impl AppWindow {
    pub fn handle_event(&self, event: AppListviewEvent) {
        match event {
            AppListviewEvent::AddItem(text) => {
                let items_model_rc = self.get_items();
                let items_model = items_model_rc
                    .as_any()
                    .downcast_ref::<VecModel<SharedString>>()
                    .expect("We know we set a VecModel earlier");
                if !text.is_empty() {
                    items_model.push(SharedString::from(text));
                    self.set_input_text(SharedString::new());
                }
            }
            // Add more event handling here as needed
        }
    }
}
