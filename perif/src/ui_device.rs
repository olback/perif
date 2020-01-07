#[derive(Clone)]
pub struct UiDevice {
    pub inner: libperif::Device,
    pub battery: Option<libperif::BatteryState>
}
