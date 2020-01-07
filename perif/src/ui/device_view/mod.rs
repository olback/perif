use gtk::prelude::*;
use crate::UiDevice;

mod battery;
mod lightning;
mod sidetone;
mod commands;
mod information;

#[derive(Clone)]
pub struct DeviceView {
    name: gtk::Label,
    battery: battery::Battery,
    lightning: lightning::Lightning,
    sidetone: sidetone::Sidetone,
    commands: commands::Commands,
    information: information::Information,
    error_output: gtk::Label
}

impl DeviceView {

    pub fn build(builder: &gtk::Builder) -> DeviceView {

        DeviceView {
            name: builder.get_object("device_name").expect("could not get device_name"),
            battery: battery::Battery::build(&builder),
            lightning: lightning::Lightning::build(&builder),
            sidetone: sidetone::Sidetone::build(&builder),
            commands: commands::Commands::build(&builder),
            information: information::Information::build(&builder),
            error_output: builder.get_object("error_output").expect("could not get error_output")
        }

    }

    pub fn show_device(&self, device: UiDevice) {

        self.name.set_text(&device.inner.name);

        match device.battery {
            Some(b) => {
                self.battery.set_battery(b);
                self.battery.set_visible(true);
            },
            None => {
                self.battery.set_visible(false);
            }
        };

        // TODO: Implement!
        match device.inner.set_lightning {
            Some(l) => {
                self.lightning.set_visible(true);
            },
            None => {
                self.lightning.set_visible(false);
            }
        };

        // TODO: Implement!
        match device.inner.set_sidetone {
            Some(s) => {
                self.sidetone.set_visible(true);
            },
            None => {
                self.sidetone.set_visible(false);
            }
        };

        // TODO: Implement!
        match device.inner.commands {
            Some(n) => {
                self.commands.set_visible(true);
            },
            None => {
                self.commands.set_visible(false);
            }
        };

        let mut info_vec = Vec::<String>::new();
        match device.inner.vid {
            Some(vid) => info_vec.push(format!("Vendor ID: {:04x}", vid)),
            None => {}
        };
        match device.inner.pid {
            Some(pid) => info_vec.push(format!("Product ID: {:04x}", pid)),
            None => {}
        };
        match device.inner.mac {
            Some(mac) => info_vec.push(format!("MAC: {}", mac)),
            None => {}
        };

        if info_vec.len() > 0 {
            let info_string = info_vec.join("\n");
            self.information.set_text(info_string);
            self.information.set_visible(true);
        } else {
            self.information.set_visible(false);
        }

    }

}
