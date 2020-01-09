use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    }
};

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

    let mut buf = [0; 51];
    let len = hid_dev.read(&mut buf[..])?;

    Ok(match len {
        // Pen is out of range (USB)
        9 => {
            let battery = buf[1];
            BatteryState::Charging(Some(battery &! 128))
        },
        // Pen is in range (USB)
        27 => {
            let battery = buf[1];
            BatteryState::Charging(Some(battery))
        },
        // Bluetooth
        51 => {
            let battery = buf[45];
            if battery & 128 != 0 {
                BatteryState::Charging(Some(battery &! 128))
            } else {
                BatteryState::Discharging(battery)
            }
        },
        _ => {
            BatteryState::Unavailable
        }
    })

}
