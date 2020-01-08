use crate::{
    device::SupportedDevice,
    device::DeviceKind
};

mod intuos;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec![
        SupportedDevice::new(
            "Wacom Intuos BT M",
            DeviceKind::Tablet,
            0x056au16,
            0x0379u16,
            Some(intuos::get_battery),
            None,
            None,
            None
        )
    ]

}
