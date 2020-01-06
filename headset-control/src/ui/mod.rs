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
    devices: Arc<Mutex<Vec<UiDevice>>>,
    main: gtk::ApplicationWindow,
    pub about_dialog: gtk::AboutDialog,
    pub devices_view: devices_view::DevicesView,
    pub stack: stack::Stack,
}

impl Ui {

    pub fn build(app: &Application) -> Self {

        let devices = Arc::new(Mutex::new(Vec::new()));

        let builder = Builder::new_from_resource(&format!("{}/ui", RESOURCE_PATH));
        let stack = stack::Stack::build(&builder, &devices);
        let dev_view = devices_view::DevicesView::build(&builder, &stack, &devices);

        let inner = Self {
            devices: devices,
            main: main::build(&builder, &app),
            about_dialog: about::build(&builder),
            devices_view: dev_view,
            stack: stack
        };

        // Connect events
        settings::Settings::connect(&builder, &inner);
        menu::Menu::connect(&builder, &inner);

        inner

    }

}
