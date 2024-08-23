pub enum AppListviewEvent {
    AddItem(),
    RemoveCheckedItems(),
    //ClickItem(),
    // Add more events here as needed
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