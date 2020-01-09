use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState
    }
};


// Does not seem to be possible to get battery, no data available to read, and no feature reports available
pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    Ok(BatteryState::Unavailable)

}
