use crate::{
    device::SupportedDevice,
    device::DeviceKind
};

mod dualshock4;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec![
        SupportedDevice::new(
            "Dualshock 4 Wireless",
            DeviceKind::Controller,
            0x054cu16,
            0x0ba0u16,
            Some(dualshock4::get_battery),
            None,
            None,
            None
        ),
        SupportedDevice::new(
            "Dualshock 4",
            DeviceKind::Controller,
            0x054cu16,
            0x05c4u16,
            Some(dualshock4::get_battery),
            None,
            None,
            None
        ),
        SupportedDevice::new(
            "Dualshock 4",
            DeviceKind::Controller,
            0x054cu16,
            0x09ccu16,
            Some(dualshock4::get_battery),
            None,
            None,
            None
        )
    ]

}
