use crate::{
    PerifResult,
    device::{
        Device,
        SupportedDevice
    }
};

// mod audio_technica;
mod corsair;
mod sony;
// mod wacom;

pub fn get_supported_devices() -> Vec<SupportedDevice> {

    vec![
        // audio_technica::supported_devices(),
        corsair::supported_devices(),
        sony::supported_devices(),
        // wacom::supported_devices()
    ].concat()

}

pub fn get_available_devices(hidapi: &mut hidapi::HidApi) -> PerifResult<Vec<Device>> {

    let mut available: Vec<Device> = Vec::new();

    hidapi.refresh_devices()?;

    for hid_dev in hidapi.devices() {
        for supported in get_supported_devices() {
            if hid_dev.vendor_id == supported.vid && hid_dev.product_id == supported.pid {
                available.push(Device {
                    name: supported.name,
                    kind: supported.kind,
                    path: hid_dev.path.clone(),
                    vid: supported.vid,
                    pid: supported.pid,
                    serial: hid_dev.serial_number.clone(),
                    manufacturer_string: hid_dev.manufacturer_string.clone(),
                    product_string: hid_dev.product_string.clone(),
                    get_battery: supported.get_battery,
                    set_lightning: supported.set_lightning,
                    set_sidetone: supported.set_sidetone,
                    commands: supported.commands
                });
                break;
            }
        }
    }

    Ok(available)

}
