use crate::{
    HCResult,
    device::{
        Device,
        BatteryState
    }
};

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> HCResult<BatteryState> {

    let hid_dev = hidapi.open(device.vid.unwrap(), device.pid.unwrap())?;

    // Request battery
    hid_dev.write(&[0xc9, 0x64])?;

    // Read response
    let mut buf = [0u8; 5];
    hid_dev.read(&mut buf[..])?;

    Ok(match buf[4] {
        1 => {
            // See https://github.com/Sapd/HeadsetControl/issues/13
            if buf[2] & 128 != 0{
                BatteryState::Discharging(buf[2] &! 128)
            } else {
                BatteryState::Discharging(buf[2])
            }
        },
        // 4 => BatteryState::Charging,
        4 => BatteryState::Full, // TODO: is this true?
        5 => BatteryState::Charging(None),
        _ => BatteryState::Unavailable,
    })

}
