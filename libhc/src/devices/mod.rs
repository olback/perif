use crate::{
    HCResult,
    device::Device
};

mod corsair;
mod sony;
mod audio_technica;

fn supported_devices() -> Vec<Device> {

    vec![
        // audio_technica::supported_devices(),
        corsair::supported_devices(),
        sony::supported_devices()
    ].concat()

}

pub fn get_available_devices(hidapi: &mut hidapi::HidApi) -> HCResult<Vec<Device>> {

    let mut available: Vec<Device> = Vec::new();

    hidapi.refresh_devices()?;

    for device in hidapi.devices() {
        for supported in supported_devices() {
            if Some(device.vendor_id) == supported.vid && Some(device.product_id) == supported.pid {
                available.push(supported);
                break;
            }
        }
    }

    Ok(available)

}
