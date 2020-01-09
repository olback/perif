use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    },
    new_err
};

// * Resources:
// * https://bitbucket.org/unessa/dualshock4-rust/src/master/
// * https://github.com/Jays2Kings/DS4Windows/blob/jay/DS4Windows/DS4Library/DS4Device.cs#L468

const DS4_BUF_LEN: usize = 64;

// TODO: Improve reliability
pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

    let mut buf = [0; DS4_BUF_LEN];

    match hid_dev.read(&mut buf[..]) {

        Ok(DS4_BUF_LEN) => {

            let charging = (buf[30] & 0x10) != 0;
            let battery = (buf[30] & 0x0f) * 10;

            Ok(match charging {
                true => BatteryState::Charging(None),
                false => BatteryState::Discharging(battery)
            })

        },
        Ok(_) => Ok(BatteryState::Unavailable),
        Err(e) => Err(new_err!(e))

    }

}
