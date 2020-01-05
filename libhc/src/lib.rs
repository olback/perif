mod macros;

mod device;
pub use device::{Device, BatteryState, CommandData, CommandFn, DeviceKind};

mod error;
pub use error::{HCError, HCResult};

mod devices;
pub use devices::get_available_devices;

// Re-export
pub use hidapi::HidApi;
