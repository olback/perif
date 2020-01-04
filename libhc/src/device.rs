use crate::HCResult;

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

type GetBatteryFn = fn(hidapi: &hidapi::HidApi, device: &Device) -> HCResult<BatteryState>;
type SetLightningFn = fn(hidapi: &hidapi::HidApi, device: &Device, state: u8) -> HCResult<bool>;
type SetSidetoneFn = fn(hidapi: &hidapi::HidApi, device: &Device, level: u8) -> HCResult<bool>;
type PlayNotificationFn = fn(hidapi: &hidapi::HidApi, device: &Device, sound: u8) -> HCResult<bool>;

#[derive(Clone)]
pub struct Device {
    pub name: String,
    pub kind: DeviceKind,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
    pub mac: Option<String>,
    pub get_battery: Option<GetBatteryFn>,
    pub set_lightning: Option<SetLightningFn>,
    pub set_sidetone: Option<SetSidetoneFn>,
    pub play_notification: Option<PlayNotificationFn>
}

// TODO: Implement this instead!
// Instead of Vec<u8>, use [u8; 8]?
// #[derive(Clone)]
// pub struct Device {
//     pub name: String,
//     pub kind: DeviceKind,
//     pub vid: Option<u16>,
//     pub pid: Option<u16>,
//     pub mac: Option<String>,
//     pub get_battery: Option<GetBatteryFn>,
//     pub set_lightning: Option<(SetLightningFn, Vec<(String, Vec<u8>)>)>,
//     pub set_sidetone: Option<(SetSidetoneFn, Vec<(String, Vec<u8>)>)>,
//     pub actions: Option<(SetSidetoneFn, Vec<(String, Vec<u8>)>)>
// }

impl Device {

    pub fn usb<N: Into<String>>(
        name: N,
        kind: DeviceKind,
        vid: u16,
        pid: u16,
        get_battery: Option<GetBatteryFn>,
        set_lightning: Option<SetLightningFn>,
        set_sidetone: Option<SetSidetoneFn>,
        play_notification: Option<PlayNotificationFn>
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
            play_notification: play_notification
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
            play_notification: None
        }

    }

}
