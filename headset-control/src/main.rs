// use libhc;

// fn main() {

//     let mut hidapi = libhc::hidapi::HidApi::new().unwrap();
//     let devices = libhc::get_available_devices(&mut hidapi).unwrap();

//     for device in devices {

//         match device.get_battery {
//             Some(get_battery) => println!("get_battery {:#?}", get_battery(&hidapi, &device)),
//             None => println!("get_battery not supported")
//         };

//         match device.set_lightning {
//             Some(set_lightning) => println!("set_lightning {:#?}", set_lightning(&hidapi, &device, 0)),
//             None => println!("set_lightning not supported")
//         };

//         match device.set_sidetone {
//             Some(set_sidetone) => println!("set_sidetone {:#?}", set_sidetone(&hidapi, &device, 0)),
//             None => println!("set_sidetone not supported")
//         };

//         match device.play_notification {
//             Some(play_notification) => println!("play_notification {:#?}", play_notification(&hidapi, &device, 0)),
//             None => println!("play_notification not supported")
//         };

//     }

// }

use gio::prelude::*;
use std::env::args;
use libhc::HidApi;
use std::sync::{Arc, Mutex};

mod utils;
mod ui;
mod consts;
mod ui_device;
mod tasks;

use ui::Ui;
use ui_device::UiDevice;

fn main() {

    // Load resources
    utils::load_resources();

    // HidApi mutex
    let hidapi: Arc<Mutex<HidApi>> = Arc::new(Mutex::new(HidApi::new().unwrap()));

    // Create app
    let application = gtk::Application::new(Some("net.olback.headset-control"), Default::default()).unwrap();
    application.connect_activate(move |app| {

        // Build UI
        let ui = Ui::build(app);
        ui.stack.show_no_devices(false);

        let hidapi_clone = hidapi.clone();
        let device_tx = ui.devices.get_tx();
        // TODO: Replace interval with value from settings
        tasks::refresh_devices(&hidapi_clone, device_tx, 2);
        tasks::command_handler(&hidapi_clone);

    });

    // Run app
    application.run(&args().collect::<Vec<_>>());

}
