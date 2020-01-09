use crate::{
    utils,
    UiDevice
};
use std::{
    thread::JoinHandle,
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicBool,
            Ordering
        }
    }
};
use libperif::HidApi;

pub fn refresh_devices(should_stop: Arc<AtomicBool>, hidapi: &Arc<Mutex<HidApi>>, device_tx: glib::Sender<Vec<UiDevice>>, interval: u64) -> JoinHandle<()> {

    let hidapi = Arc::clone(&hidapi);

    std::thread::spawn(move || {

        loop {

            if should_stop.load(Ordering::Acquire) {
                println!("Stopping device thread...");
                return;
            }

            let ui_devices = utils::safe_lock(&hidapi, |hid_lock| {

                let mut ui_devices: Vec<UiDevice> = Vec::new();

                let devices = libperif::get_available_devices(&mut *hid_lock).unwrap();

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

    })

}
