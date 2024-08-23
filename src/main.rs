mod handlers;
mod ui_modules;
use crate::handlers::app_listview;
use crate::ui_modules::AppWindow;
use slint::Weak;

use slint::ComponentHandle;
use std::sync::{mpsc, Arc, Mutex};
use std::thread; // Use the re-exported AppWindow type

pub enum UIEvent {
    AddItem(),
    RemoveCheckedItems(),
    // Add more events here as needed
}

// The format of the messages that we'll pass to the worker thread
type Message = (Arc<Mutex<Weak<AppWindow>>>, UIEvent);

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Initialize channel to pass messages to worker thread
    let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

    // Initialize worker thread
    let _worker_thread_handle = thread::spawn(move || {
        // Loop over incoming messages
        while let Ok((app_window, event)) = rx.recv() {
            match event {
                UIEvent::AddItem() => app_listview::handle_add_item(app_window),
                UIEvent::RemoveCheckedItems() => {
                    app_listview::handle_remove_checked_items(app_window)
                }
            }
        }
    });

    let app_listview_tx = tx.clone();
    app_listview::init(&ui, app_listview_tx);

    ui.run()
}
