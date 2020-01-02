use crate::{
    HCResult,
    device::Device
};

mod corsair;

fn supported_devices() -> Vec<Device> {

    vec![
        Device::usb(
            "Corsair Void Pro Wireless",
            0x1b1cu16,
            0x0a14u16,
            Some(corsair::void::get_battery),
            None,
            None,
            None
        )
    ]

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
