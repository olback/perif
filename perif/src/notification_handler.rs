use std::{
    ffi::CString,
    collections::HashMap,
    time::{
        SystemTime,
        UNIX_EPOCH
    }
};
use crate::{
    ui_device::UiDevice
};
use notify_rust::Notification;
use libperif::{
    PerifResult,
    new_err,
    BatteryState
};

pub struct NotificationHandler {
    devices: HashMap<CString, u64>,
    interval: u64
}

impl NotificationHandler {

    pub fn new(interval: u64) -> NotificationHandler {

        NotificationHandler {
            devices: HashMap::<CString, u64>::new(),
            interval: interval
        }

    }

    pub fn show(&mut self, device: &UiDevice, level: u8) {

        let inner_device = self.devices.get_mut(&device.inner.path);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("could not get unix epoch time, check system time");
        let epoch = now.as_secs();

        let should_send = match inner_device {
            Some(v) => {
                let r = *v + (self.interval * 60) >= epoch;
                *v = epoch;
                r
            },
            None => {
                self.devices.insert(device.inner.path.clone(), epoch);
                true
            }
        };

        if should_send {

            match NotificationHandler::send(&device.inner.name, level) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            };

        }

    }

    fn send(name: &String, level: u8) -> PerifResult<()> {

        let notif = Notification::new()
            .summary(name)
            .body(&format!("Battery is getting low! ({}%)", level))
            .icon("net.olback.Perif")
            .show();

        match notif {
            Ok(_) => Ok(()),
            Err(e) => Err(new_err!(e))
        }

    }

}
