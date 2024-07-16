use slint::Model;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let items_model: Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
    // Convert it to a ModelRc.
    let items_model_rc = ModelRc::from(items_model.clone());
    // Pass the model to the ui: The generated set_the_model setter from the
    // the_model property takes a ModelRc.
    ui.set_items(items_model_rc);
    let ui_weak = ui.as_weak();
    ui.on_request_add_item(move || {
            let app = ui_weak.unwrap();
            let items_model_rc = app.get_items();
            let input_text = app.get_input_text();
            let items_model = items_model_rc
                .as_any()
                .downcast_ref::<VecModel<SharedString>>()
                .expect("We know we set a VecModel earlier");
            if !input_text.is_empty() {
                items_model.push(SharedString::from(input_text));
                app.set_input_text(SharedString::new());
            }
    });

    ui.run()
}