#[derive(Clone)]
pub struct UiDevice {
    pub inner: libhc::Device,
    pub battery: Option<libhc::BatteryState>
}
