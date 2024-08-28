mod data_structs;
mod events;
mod handlers;
mod io;
mod ui_modules;
use slint::ComponentHandle;
use slint::Weak;
use std::sync::{mpsc, Arc, Mutex};

use crate::events::*;
use crate::handlers::app_listview;
use crate::handlers::app_tabwidget;
use crate::handlers::ui_worker;
use crate::ui_modules::AppWindow; // Use the re-exported AppWindow type

fn main() -> Result<(), slint::PlatformError> {
    type Message = (Arc<Mutex<Weak<AppWindow>>>, UIEvent);
    let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let listview_tx = tx.clone();
    let tabwidget_tx = tx.clone();

    // Load from disk
    let _ = io::load();

    let ui = AppWindow::new()?;
    ui_worker::init(rx);
    app_listview::init(&ui, listview_tx);
    app_tabwidget::init(&ui, tabwidget_tx);

    ui.run()
}
