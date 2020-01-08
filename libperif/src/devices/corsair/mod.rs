use crate::{
    device::SupportedDevice,
    device::DeviceKind
};

mod void;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec![
        SupportedDevice::new(
            "Corsair Void Pro Wireless",
            DeviceKind::Headset,
            0x1b1cu16,
            0x0a14u16,
            Some(void::get_battery),
            None,
            None,
            None
        )
    ]

}
