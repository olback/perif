use crate::PerifResult;

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
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub mac: Option<String>,
    pub get_battery: Option<GetBatteryFn>,
    pub set_lightning: Option<(CommandFn, Vec<(String, CommandData)>)>,
    pub set_sidetone: Option<CommandFn>,
    pub commands: Option<(CommandFn, Vec<(String, CommandData)>)>
}

impl Device {

    pub fn usb<N: Into<String>>(
        name: N,
        kind: DeviceKind,
        vid: u16,
        pid: u16,
        get_battery: Option<GetBatteryFn>,
        set_lightning: Option<(CommandFn, Vec<(String, CommandData)>)>,
        set_sidetone: Option<CommandFn>,
        commands: Option<(CommandFn, Vec<(String, CommandData)>)>
    ) -> Device {
        Device {
            name: name.into(),
            kind: kind,
            vid: Some(vid.into()),
            pid: Some(pid.into()),
            mac: None,
            get_battery: get_battery,
            set_lightning: set_lightning,
            set_sidetone: set_sidetone,
            commands: commands
        }
    }

    pub fn bluetooth<N: Into<String>, M: Into<String>>(name: N, mac: M) -> Device {

        Device {
            name: name.into(),
            kind: DeviceKind::Bluetooth,
            vid: None,
            pid: None,
            mac: Some(mac.into()),
            get_battery: None,
            set_lightning: None,
            set_sidetone: None,
            commands: None
        }

    }

}
