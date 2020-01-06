use crate::{
    device::Device,
    device::DeviceKind
};

mod ath_g1wl;

pub fn supported_devices() -> Vec<Device> {

    vec![
        Device::usb(
            "Audio Technica ATH-G1WL",
            DeviceKind::Headset,
            0x0451u16,
            0x16bau16,
            Some(ath_g1wl::get_battery),
            None,
            None,
            None
        )
    ]

}
