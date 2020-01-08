use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    }
};


pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

    // hid_dev.write(&[0x00, 0xea])?;

    let mut buf = [0; 32];
    let len = hid_dev.read_timeout(&mut buf[..], 100)?;

    println!("Length: {}\n{:?}", len, &buf[0..len]);

    Ok(BatteryState::Unavailable)

}