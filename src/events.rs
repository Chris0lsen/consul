use slint::SharedString;

pub enum AppListviewEvent {
    AddItem(),
    RemoveCheckedItems(),
    SaveItem(usize, SharedString), // Add more events here as needed
}

pub enum AppTabWidgetEvent {
    AddItem(),
    RemoveItem(),
    // Add more events here as needed
}

pub enum UIEvent {
    AppListView(AppListviewEvent),
    AppTabWidget(AppTabWidgetEvent),
}
