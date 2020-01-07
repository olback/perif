use gtk::{
    Application,
    Builder
};
use crate::{
    consts::RESOURCE_PATH,
    ui_device::UiDevice
};
use std::sync::{
    Arc,
    Mutex
};

mod main;
mod about;
mod stack;
mod settings;
mod devices_view;
mod menu;
mod device_view;

pub struct Ui {
    pub about_dialog: gtk::AboutDialog,
    pub devices_view: devices_view::DevicesView,
    pub stack: stack::Stack,
}

impl Ui {

    pub fn build(app: &Application, gio_settings: gio::Settings) -> Self {

        let devices = Arc::new(Mutex::new(Vec::<UiDevice>::new()));

        let builder = Builder::new_from_resource(&format!("{}/ui", RESOURCE_PATH));
        let stack = stack::Stack::build(&builder, &devices);
        let dev_view = devices_view::DevicesView::build(&builder, &stack, &devices);

        let inner = Self {
            about_dialog: about::build(&builder),
            devices_view: dev_view,
            stack: stack
        };

        // Build main window
        main::build(&builder, &app);

        // Connect events
        settings::Settings::connect(&builder, &inner, gio_settings);
        menu::Menu::connect(&builder, &inner);

        inner

    }

}
