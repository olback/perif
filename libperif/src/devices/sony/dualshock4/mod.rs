use crate::{
    PerifResult,
    device::{
        Device,
        BatteryState,
        CommandData,
        SupportedDevice,
        DeviceKind
    },
    new_err
};

mod ds4_writer;
use ds4_writer::ToDS4Writer;


// * Resources:
// * https://bitbucket.org/unessa/dualshock4-rust/src/master/
// * https://github.com/Jays2Kings/DS4Windows/blob/jay/DS4Windows/DS4Library/DS4Device.cs#L468
// * https://github.com/kpeu3i/gods4/blob/081d376c7200ad942f66092e5536a64af6b4e1ab/controller.go#L278

// Command data                       [cid, type, intensity, time]
pub const RUMBLE_LIGHT:      &[u8] = &[0x00, 0x00, 0xff, 200];
pub const RUMBLE_HEAVY:      &[u8] = &[0x00, 0x01, 0xff, 200];

// r, g, b, on, off
// To make the light blink with a duty cycle of 50%, one would set on: 0x32 (50) and off: 0x32 (50)
// To make it 30% on and 70% off: on: 0x1e (30) and off: 0x46 (70).
// Lightning data                     [r,    g,    b,    on,   off ]
pub const LIGHTNING_OFF:     &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00];
pub const LIGHTNING_RED:     &[u8] = &[0xff, 0x00, 0x00, 0x00, 0x00];
pub const LIGHTNING_GREEN:   &[u8] = &[0x00, 0x80, 0x00, 0x00, 0x00];
pub const LIGHTNING_BLUE:    &[u8] = &[0x00, 0x00, 0xff, 0x00, 0x00];
pub const LIGHTNING_LIME:    &[u8] = &[0x00, 0xff, 0x00, 0x00, 0x00];
pub const LIGHTNING_YELLOW:  &[u8] = &[0xff, 0xff, 0x00, 0x00, 0x00];
pub const LIGHTNING_CYAN:    &[u8] = &[0x00, 0xff, 0xff, 0x00, 0x00];
pub const LIGHTNING_MAGENTA: &[u8] = &[0xff, 0x00, 0xff, 0x00, 0x00];
pub const LIGHTNING_SILVER:  &[u8] = &[0xc0, 0xc0, 0xc0, 0x00, 0x00];
pub const LIGHTNING_GRAY:    &[u8] = &[0x80, 0x80, 0x80, 0x00, 0x00];
pub const LIGHTNING_MAROON:  &[u8] = &[0x80, 0x00, 0x00, 0x00, 0x00];
pub const LIGHTNING_OLIVE:   &[u8] = &[0x80, 0x80, 0x00, 0x00, 0x00];
pub const LIGHTNING_PURPLE:  &[u8] = &[0x80, 0x00, 0x80, 0x00, 0x00];
pub const LIGHTNING_TEAL:    &[u8] = &[0x00, 0x80, 0x80, 0x00, 0x00];
pub const LIGHTNING_NAVY:    &[u8] = &[0x00, 0x00, 0x80, 0x00, 0x00];

const DS4_INPUT_LENGTH: usize = 100;
const DS4_OUTPUT_LENGTH: usize = 79;
const DS4_USB_INPUT_LENGTH: usize = 64;
const DS4_BT_INPUT_LENGTH: usize = 78;
const USB_OFFSET: (usize, usize) = (0, 0);
const BT_OFFSET: (usize, usize) = (2, 3);

pub fn get_battery(hidapi: &hidapi::HidApi, device: &Device) -> PerifResult<BatteryState> {

    let hid_dev = hidapi.open_path(&device.path)?;

    let mut buf = [0; DS4_INPUT_LENGTH];

    match hid_dev.read_timeout(&mut buf[..], 100) {

        // USB
        Ok(DS4_USB_INPUT_LENGTH) => {

            let (io, _) = USB_OFFSET;

            let charging = (buf[30 + io] & 0x10) != 0;
            let battery = (buf[30 + io] & 0x0f) * 10;

            Ok(match charging {
                true => BatteryState::Charging(None),
                false => BatteryState::Discharging(battery)
            })

        },

        // BT
        Ok(DS4_BT_INPUT_LENGTH) => {

            let (io, _) = BT_OFFSET;

            let charging = (buf[30 + io] & 0x10) != 0;
            let battery = (buf[30 + io] & 0x0f) * 10;

            Ok(match charging {
                true => BatteryState::Charging(None),
                false => BatteryState::Discharging(battery)
            })

        },

        Ok(_) => Ok(BatteryState::Unavailable),
        Err(e) => Err(new_err!(e))

    }

}

// Data structure
// [0] red
// [1] green
// [2] blue
pub fn set_lightning(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<()> {

    let new_data = [vec![0xff], data].concat();

    commands(hidapi, device, new_data)

}

// Data structure
// [0] Command
// [1..n] data
pub fn commands(hidapi: &hidapi::HidApi, device: &Device, data: CommandData) -> PerifResult<()> {

    // Output buffer
    let mut output_state = [0; DS4_OUTPUT_LENGTH];

    // Open hid device
    let ds4_dev = hidapi.open_ds4(&device.path)?;

    // if  ds4_dev.is_bluetooth {
    //     output_state[0] = 0xa2;
    //     output_state[1] = 0x11;
    //     output_state[2] = 0x80;
    //     output_state[4] = 0x0f;
    // } else {
    //     output_state[0] = 0x05;
    //     output_state[1] = 0x07;
    // }

    match data[0] {

        // Rumble data structure
        // [1] 0 = light, 1 = heavy
        // [2] Intensity
        // [3] Time in ms (0-255)
        0x00 => {

            if data.len() == 4 {

                // 4 light
                // 5 heavy
                match data[1] {
                    0 => output_state[4 + ds4_dev.output_offset] = data[2],
                    1 => output_state[5 + ds4_dev.output_offset] = data[2],
                    _ => return Err(new_err!("Invalid command data"))
                };

                // Send data
                ds4_dev.write(&mut output_state)?;

                // Reset rumble
                output_state[4 + ds4_dev.output_offset] = 0;
                output_state[5 + ds4_dev.output_offset] = 0;

                // Wait specified time
                std::thread::sleep(std::time::Duration::from_millis(data[3] as u64));

                // Tell the controller to stop
                ds4_dev.write(&mut output_state)?;

                Ok(())

            } else {

                Err(new_err!("Invalid command data"))

            }

        },

        // Color data structure
        // [1] red
        // [2] green
        // [3] blue
        0xff => {

            if data.len() == 6 {

                // Set colors
                output_state[6 + ds4_dev.output_offset] = data[1];
                output_state[7 + ds4_dev.output_offset] = data[2];
                output_state[8 + ds4_dev.output_offset] = data[3];
                output_state[9 + ds4_dev.output_offset] = data[4];
                output_state[10 + ds4_dev.output_offset] = data[5];

                // Send data
                ds4_dev.write(&mut output_state)?;

                Ok(())

            } else {

                Err(new_err!("Invalid command data"))

            }

        },

        _ => Err(new_err!("Invalid command"))

    }

}

// A helper function to make adding a new DS4 Controller easier.
pub fn new(name: &str, pid: u16) -> SupportedDevice {

    SupportedDevice::new(
        String::from(name),
        DeviceKind::Controller,
        0x054cu16,
        pid,
        Some(get_battery),
        Some((set_lightning, vec![
            SupportedDevice::option("Off", LIGHTNING_OFF),
            SupportedDevice::option("Red", LIGHTNING_RED),
            SupportedDevice::option("Green", LIGHTNING_GREEN),
            SupportedDevice::option("Blue", LIGHTNING_BLUE),
            SupportedDevice::option("Lime", LIGHTNING_LIME),
            SupportedDevice::option("Yellow", LIGHTNING_YELLOW),
            SupportedDevice::option("Cyan", LIGHTNING_CYAN),
            SupportedDevice::option("Magenta", LIGHTNING_MAGENTA),
            SupportedDevice::option("Silver", LIGHTNING_SILVER),
            SupportedDevice::option("Gray", LIGHTNING_GRAY),
            SupportedDevice::option("Maroon", LIGHTNING_MAROON),
            SupportedDevice::option("Olive", LIGHTNING_OLIVE),
            SupportedDevice::option("Purple", LIGHTNING_PURPLE),
            SupportedDevice::option("Teal", LIGHTNING_TEAL),
            SupportedDevice::option("Navy", LIGHTNING_NAVY)
        ])),
        None,
        Some((commands, vec![
            SupportedDevice::option("Light rumble", RUMBLE_LIGHT),
            SupportedDevice::option("Heavy rumble", RUMBLE_HEAVY)
        ]))
    )

}
