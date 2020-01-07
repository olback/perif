use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    },
    new_err
};


pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open(device.vid.unwrap(), device.pid.unwrap())?;

    // hid_dev.write(&[0x00, 0xea])?;

    let mut buf = [0; 32];
    let len = hid_dev.read_timeout(&mut buf[..], 100)?;

    println!("Length: {}\n{:?}", len, &buf[0..len]);

    Ok(BatteryState::Unavailable)

}
