use crate::{
    utils,
    UiDevice
};
use std::sync::{
    Arc,
    Mutex
};
use libhc::HidApi;

pub fn refresh_devices(hidapi: &Arc<Mutex<HidApi>>, device_tx: glib::Sender<Vec<UiDevice>>, interval: u64) {

    let hidapi = Arc::clone(&hidapi);

    std::thread::spawn(move || {

        loop {

            let ui_devices = utils::safe_lock(&hidapi, |hid_lock| {

                let devices = libhc::get_available_devices(&mut *hid_lock).unwrap();

                let mut ui_devices: Vec<UiDevice> = Vec::new();

                for dev in devices {

                    let battery = match dev.get_battery {
                        Some(get_battery) => match get_battery(&hid_lock, &dev) {
                            Ok(b) => Some(b),
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            }
                        },
                        None => None
                    };

                    println!("{:#?}", battery);

                    ui_devices.push(UiDevice {
                        inner: dev,
                        battery: battery
                    });

                }

                ui_devices

            });

            device_tx.send(ui_devices).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(interval));

        }

    });



}
