mod handlers;
mod ui;
mod ui_events;
use slint::ComponentHandle;

use crate::ui::AppWindow; // Use the re-exported AppWindow type
use crate::handlers::app_listview;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    app_listview::init(&ui);

    ui.run()
}
