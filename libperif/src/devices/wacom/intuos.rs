use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    }
};

// TODO: WIP
pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

    let mut buf = [0; 256];

    let len = hid_dev.read(&mut buf[..])?;
    println!("{} : {:#?}", len, &buf[0..len]);

    Ok(BatteryState::Unavailable)

}
