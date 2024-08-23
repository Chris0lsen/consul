mod handlers;
mod ui_modules;
use slint::ComponentHandle;

use crate::handlers::app_tabwidget;
use crate::handlers::app_listview;
use crate::ui_modules::AppWindow; // Use the re-exported AppWindow type

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    app_listview::init(&ui);

    app_tabwidget::init(&ui);

    ui.run()
}
