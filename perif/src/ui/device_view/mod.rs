use gtk::prelude::*;
use crate::{
    UiDevice,
    tasks::{Command, CommandResult}
};
use std::sync::mpsc;

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
    result_tx: glib::Sender<CommandResult>
}

impl DeviceView {

    pub fn build(builder: &gtk::Builder) -> DeviceView {

        // Handle close event
        let result_infobar: gtk::InfoBar = builder.get_object("result_infobar").expect("could not get result_infobar");
        let result_output: gtk::Label = builder.get_object("result_output").expect("could not get result_output");
        result_infobar.connect_response(|info_bar, _| {
            info_bar.set_visible(false);
        });

        let (tx, rx) = glib::MainContext::channel::<CommandResult>(glib::PRIORITY_DEFAULT);
        rx.attach(None, move |res| {

            match res {
                CommandResult::Success(msg) => {
                    result_infobar.set_message_type(gtk::MessageType::Info);
                    result_output.set_text(msg.as_str());
                },
                CommandResult::Error(msg) => {
                    result_infobar.set_message_type(gtk::MessageType::Error);
                    result_output.set_text(msg.as_str());
                }
            }

            result_infobar.set_visible(true);

            glib::Continue(true)

        });

        DeviceView {
            name: builder.get_object("device_name").expect("could not get device_name"),
            battery: battery::Battery::build(&builder),
            lightning: lightning::Lightning::build(&builder),
            sidetone: sidetone::Sidetone::build(&builder),
            commands: commands::Commands::build(&builder),
            information: information::Information::build(&builder),
            result_tx: tx
        }

    }

    pub fn get_tx(&self) -> glib::Sender<CommandResult> {

        self.result_tx.clone()

    }

    pub fn show_device(&self, device: UiDevice, update: bool) {

        // Always update battery
        match &device.battery {
            Some(b) => {
                self.battery.set_battery(b.clone());
                self.battery.set_visible(true);
            },
            None => {
                self.battery.set_visible(false);
            }
        };

        if update {

            self.name.set_text(&device.inner.name);

            match &device.inner.set_lightning {
                Some(l) => {
                    self.lightning.set_visible(true);
                    self.lightning.set_commands(l.clone(), device.clone());
                },
                None => {
                    self.lightning.set_visible(false);
                }
            };

            match &device.inner.set_sidetone {
                Some(s) => {
                    self.sidetone.set_visible(true);
                    self.sidetone.set_command(s.clone(), device.clone());
                },
                None => {
                    self.sidetone.set_visible(false);
                }
            };

            match &device.inner.commands {
                Some(n) => {
                    self.commands.set_visible(true);
                    self.commands.set_commands(n.clone(), device.clone());
                },
                None => {
                    self.commands.set_visible(false);
                }
            };

            let mut info_vec = Vec::<String>::new();
            info_vec.push(format!("Path: {}", device.inner.path.into_string().unwrap()));
            info_vec.push(format!("Vendor ID: {:04x}", device.inner.vid));
            info_vec.push(format!("Product ID: {:04x}", device.inner.pid));

            match device.inner.serial {
                Some(serial) => if serial.len() > 0 { info_vec.push(format!("Serial: {}", serial)) },
                None => {}
            }

            match device.inner.manufacturer_string {
                Some(mfr) => if mfr.len() > 0 { info_vec.push(format!("Manufacturer: {}", mfr)) },
                None => {}
            }

            match device.inner.product_string {
                Some(prd) => if prd.len() > 0 { info_vec.push(format!("Product: {}", prd)) },
                None => {}
            }

            if info_vec.len() > 0 {
                let info_string = info_vec.join("\n");
                self.information.set_text(info_string);
                self.information.set_visible(true);
            } else {
                self.information.set_visible(false);
            }

        }

    }

    pub fn connect_lightning(&mut self, command_tx: mpsc::Sender<Option<Command>>) {

        self.lightning.connect(command_tx);

    }

    pub fn connect_sidetone(&mut self, command_tx: mpsc::Sender<Option<Command>>) {

        self.sidetone.connect(command_tx);

    }

    pub fn connect_commands(&mut self, command_tx: mpsc::Sender<Option<Command>>) {

        self.commands.connect(command_tx);

    }

}
