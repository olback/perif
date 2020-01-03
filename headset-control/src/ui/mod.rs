use gtk::{
    Application,
    Builder
};
use crate::consts::RESOURCE_PATH;

mod main;
mod about;
mod stack;
mod settings;
mod devices;
mod menu;
mod device_view;

pub struct Ui {
    main: gtk::ApplicationWindow,
    pub about_dialog: gtk::AboutDialog,
    pub devices: devices::Devices,
    pub stack: stack::Stack,
}

impl Ui {

    pub fn build(app: &Application) -> Self {

        let builder = Builder::new_from_resource(&format!("{}/ui", RESOURCE_PATH));
        let stack = stack::Stack::build(&builder);

        let inner = Self {
            main: main::build(&builder, &app),
            about_dialog: about::build(&builder),
            devices: devices::Devices::build(&builder, &stack),
            stack: stack
        };

        // Connect events
        settings::Settings::connect(&builder, &inner);
        menu::Menu::connect(&builder, &inner);

        inner

    }

}
