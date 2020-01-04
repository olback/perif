use crate::{
    device::Device,
    device::DeviceKind
};

mod ds4;

pub fn supported_devices() -> Vec<Device> {

    vec![
        Device::usb(
            "Dualshock 4 Wireless",
            DeviceKind::Controller,
            0x054cu16,
            0x0ba0u16,
            Some(ds4::get_battery),
            None,
            None,
            None
        ),
        Device::usb(
            "Dualshock 4",
            DeviceKind::Controller,
            0x054cu16,
            0x05c4u16,
            Some(ds4::get_battery),
            None,
            None,
            None
        )
    ]

}
