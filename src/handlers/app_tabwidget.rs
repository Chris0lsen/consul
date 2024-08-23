use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::ui_modules::TaskItem;
use crate::ui_modules::TabItem;
use slint::ModelRc;
use slint::Weak;
use slint::VecModel;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::handlers::ui_enum::*;


pub fn init(ui: &AppWindow, tx: Sender<(Arc<Mutex<Weak<AppWindow>>>, UIEvent)>) {
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
}