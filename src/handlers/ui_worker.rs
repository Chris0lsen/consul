use crate::ui_modules::AppWindow; // Import the re-exported AppWindow type
                                  // use crate::ui_events::UIEvent; // Import the UIEvent enum
use crate::events::*;
use crate::handlers::app_listview;
use slint::Weak;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use super::app_tabwidget;

pub fn init(rx: Receiver<(Arc<Mutex<Weak<AppWindow>>>, UIEvent)>) {
    // Initialize worker thread
    let _worker_thread_handle = thread::spawn(move || {
        // Loop over incoming messages
        while let Ok((app_window, event)) = rx.recv() {
            match event {
                UIEvent::AppListView(event) => app_listview::handle_event(app_window, event),
                UIEvent::AppTabWidget(event) => app_tabwidget::handle_event(app_window, event), // Add more event handling here as needed
                                                                                                // AppListviewEvent::ClickItem() => handle_click_item(&app_window),
            }
        }
    });
}
