use crate::{
    HCResult,
    device::{
        Device,
        BatteryState
    }
};

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> HCResult<BatteryState> {

    let hid_dev = hidapi.open(device.vid.unwrap(), device.pid.unwrap())?;

    // Write data to device
    let buf = [0xc9, 0x64];
    let res = hid_dev.write(&buf)?;
    println!("Wrote: {:?} byte(s)", res);

    // Read data from device
    let mut buf = [0u8; 5];
    let res = hid_dev.read(&mut buf[..])?;
    println!("Read: {:?}", &buf[..res]);

    Ok(match buf[4] {
        1 => {
            if buf[2] & 128 != 0{
                BatteryState::Discharging(buf[2] &! 128)
            } else {
                BatteryState::Discharging(buf[2])
            }
        },
        4 => BatteryState::Charging,
        5 => BatteryState::Charging,
        _ => BatteryState::Unavailable,
    })

}
