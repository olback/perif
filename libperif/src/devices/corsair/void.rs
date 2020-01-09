use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState,
        CommandData
    },
    new_err,
    utils
};

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

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
        4 => BatteryState::Full,
        5 => BatteryState::Charging(None),
        _ => BatteryState::Unavailable,
    })

}

pub fn set_lightning(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<()> {

    commands(&hidapi, &device, data)

}

pub fn set_sidetone(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<()> {

    if data.len() == 1 {

        let sidetone_level = utils::map(data[0] as u32, 0, 255, 200, 255) as u8;
        let data = [ 0xff, 0x0b, 0x00, 0xff, 0x04, 0x0e, 0xff, 0x05, 0x01, 0x04, 0x00, sidetone_level ];

        let hid_dev = hidapi.open_path(&device.path)?;
        hid_dev.send_feature_report(&data)?;


    } else {

        return Err(new_err!("Invalid sidetone data"))

    }

    Ok(())

}

pub fn commands(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<()> {

    let hid_dev = hidapi.open_path(&device.path)?;

    // Send data
    match hid_dev.write(&data) {
        Ok(_) => Ok(()),
        Err(e) => Err(new_err!(e))
    }

}

#[macro_use]
macro_rules! corsair_void {
    ($name:expr, $pid:expr, $b:expr) => {

        SupportedDevice::new(
            $name,
            DeviceKind::Headset,
            0x1b1cu16,
            $pid,
            match $b {
                true => Some(void::get_battery),
                false => None
            },
            Some((void::set_lightning, vec![
                SupportedDevice::option("RGB On", &[0xc8, 0x00, 0x00]),
                SupportedDevice::option("RGB Off", &[0xc8, 0x01, 0x00])
            ])),
            Some(void::set_sidetone),
            Some((void::commands, vec![
                SupportedDevice::option("Play sound 1", &[0xca, 0x02, 0x00]),
                SupportedDevice::option("Play sound 2", &[0xca, 0x02, 0x01])
            ]))
        )
    };
}
