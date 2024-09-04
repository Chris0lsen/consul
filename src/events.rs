use slint::SharedString;

pub enum AppListviewEvent {
    AddItem(),
    RemoveCheckedItems(),
    SaveItem(usize, SharedString), // Add more events here as needed
}

pub enum AppTabWidgetEvent {
    AddTab(),
    RemoveTab(),
    // Add more events here as needed
}

pub enum UIEvent {
    AppListView(AppListviewEvent),
    AppTabWidget(AppTabWidgetEvent),
}
