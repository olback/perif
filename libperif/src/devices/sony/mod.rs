use crate::device::SupportedDevice;

mod dualshock4;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec![
        dualshock4::new("Sony Dualshock 4 Wireless", 0x0ba0u16),
        dualshock4::new("Sony Dualshock 4", 0x05c4u16),
        dualshock4::new("Sony Dualshock 4", 0x09ccu16)
    ]

}
