use crate::PerifResult;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum DeviceKind {
    Bluetooth,
    Controller,
    Headphones,
    Headset,
    Mouse,
    Keyboard,
    Tablet
}

#[derive(Debug, Clone)]
pub enum BatteryState {
    Discharging(u8),
    Charging(Option<u8>),
    Full,
    Unavailable
}

type GetBatteryFn = fn(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState>;
pub type CommandData = [u8; 8];
pub type CommandFn = fn(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<bool>;

#[derive(Clone)]
pub struct Device {
    pub name: String,
    pub kind: DeviceKind,
    pub path: CString,
    pub vid: u16,
    pub pid: u16,
    pub serial: Option<String>,
    pub manufacturer_string: Option<String>,
    pub product_string: Option<String>,
    pub get_battery: Option<GetBatteryFn>,
    pub set_lightning: Option<(CommandFn, Vec<(String, CommandData)>)>,
    pub set_sidetone: Option<CommandFn>,
    pub commands: Option<(CommandFn, Vec<(String, CommandData)>)>
}

#[derive(Clone)]
pub struct SupportedDevice {
    pub name: String,
    pub kind: DeviceKind,
    pub vid: u16,
    pub pid: u16,
    pub get_battery: Option<GetBatteryFn>,
    pub set_lightning: Option<(CommandFn, Vec<(String, CommandData)>)>,
    pub set_sidetone: Option<CommandFn>,
    pub commands: Option<(CommandFn, Vec<(String, CommandData)>)>
}

impl SupportedDevice {

    pub fn new<N: Into<String>>(
        name: N,
        kind: DeviceKind,
        vid: u16,
        pid: u16,
        get_battery: Option<GetBatteryFn>,
        set_lightning: Option<(CommandFn, Vec<(String, CommandData)>)>,
        set_sidetone: Option<CommandFn>,
        commands: Option<(CommandFn, Vec<(String, CommandData)>)>
    ) -> SupportedDevice {
        SupportedDevice {
            name: name.into(),
            kind: kind,
            vid: vid.into(),
            pid: pid.into(),
            get_battery: get_battery,
            set_lightning: set_lightning,
            set_sidetone: set_sidetone,
            commands: commands
        }
    }

}
