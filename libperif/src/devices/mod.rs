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
mod wacom;

pub fn get_supported_devices() -> Vec<SupportedDevice> {

    vec![
        // audio_technica::supported_devices(),
        corsair::supported_devices(),
        sony::supported_devices(),
        wacom::supported_devices()
    ].concat()

}

pub fn get_available_devices(hidapi: &mut hidapi::HidApi) -> PerifResult<Vec<Device>> {

    let mut available: Vec<Device> = Vec::new();

    hidapi.refresh_devices()?;

    for hid_dev in hidapi.device_list() {
        for supported in get_supported_devices() {
            if hid_dev.vendor_id() == supported.vid && hid_dev.product_id() == supported.pid {
                available.push(Device {
                    name: supported.name,
                    kind: supported.kind,
                    path: hid_dev.path().to_owned(),
                    vid: supported.vid,
                    pid: supported.pid,
                    serial: hid_dev.serial_number().map_or(None, |s| Some(s.to_owned())),
                    manufacturer_string: hid_dev.manufacturer_string().map_or(None, |s| Some(s.to_owned())),
                    product_string: hid_dev.product_string().map_or(None, |s| Some(s.to_owned())),
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
