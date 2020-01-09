mod macros;

mod device;
pub use device::{Device, BatteryState, CommandData, CommandFn, DeviceKind};

mod error;
pub use error::{PerifError, PerifResult};

mod devices;
pub use devices::get_available_devices;

pub mod utils;

// Re-export
pub use hidapi::HidApi;
