use crate::{
    device::SupportedDevice,
    device::DeviceKind
};

#[macro_use] mod void;

pub fn supported_devices() -> Vec<SupportedDevice> {

    vec!
    [
        corsair_void!("Corsair Void Wireless", 0x1b27, true),
        corsair_void!("Corsair Void Pro", 0x0a14, true),
        corsair_void!("Corsair Void Pro R2", 0x0a16, true),
        corsair_void!("Corsair Void Pro USB", 0x0a17, false),
        corsair_void!("Corsair Void Pro Wireless", 0x0a1a, true),
        corsair_void!("Corsair Void RGB USB", 0x1b2a, false),
        corsair_void!("Corsair Void RGB USB 2", 0x1b23, false),
        corsair_void!("Corsair Void RGB Wired", 0x1b1c, false),
    ]

}
