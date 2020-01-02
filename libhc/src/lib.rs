mod device;
mod macros;

mod error;
pub use error::{HCError, HCResult};

mod devices;
pub use devices::get_available_devices;

// Re-export
pub use hidapi;
