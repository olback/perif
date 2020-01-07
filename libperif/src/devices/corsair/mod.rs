use crate::{
    device::Device,
    device::DeviceKind
};

mod void;

pub fn supported_devices() -> Vec<Device> {

    vec![
        Device::usb(
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
