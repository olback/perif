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
use gtk::prelude::*;
use std::env::args;

mod utils;
mod ui;
use ui::Ui;

fn main() {

    utils::load_resources();

    let application = gtk::Application::new(Some("net.olback.headset-control"), Default::default()).unwrap();

    application.connect_activate(move |app| {

        let ui = Ui::build(app);

    });

    // Run app
    application.run(&args().collect::<Vec<_>>());

}
