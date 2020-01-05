use crate::{
    HCResult,
    device::{
        Device,
        BatteryState
    },
    new_err
};

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> HCResult<BatteryState> {

    let hid_dev = hidapi.open(device.vid.unwrap(), device.pid.unwrap())?;

    // TODO: Request battery
    // hid_dev.write(&[0xc9, 0x64])?;

    // Read response
    let mut buf = [0u8; 64];
    hid_dev.read(&mut buf[..])?;

    if buf[0] == 0x00 {
        return Err(new_err!("Invalid data"));
    }

    // https://github.com/Jays2Kings/DS4Windows/blob/jay/DS4Windows/DS4Library/DS4Device.cs#L468
    let charging = (buf[30] & 0x10) != 0;
    let battery = (buf[30] & 0x0f) * 10;

    Ok(match charging {
        true => BatteryState::Charging(Some(battery)),
        false => BatteryState::Discharging(battery)
    })

}
