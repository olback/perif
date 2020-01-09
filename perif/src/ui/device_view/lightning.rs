use gtk::prelude::*;
use libperif::{CommandData, CommandFn};
use std::sync::{
    Arc,
    Mutex,
    mpsc
};
use crate::{
    utils,
    tasks::Command,
    ui_device::UiDevice
};

#[derive(Clone)]
pub struct Lightning {
    inner: gtk::Box,
    combo_box: gtk::ComboBoxText,
    apply: gtk::Button,
    commands: Arc<Mutex<Option<(CommandFn, Vec<(String, CommandData)>)>>>,
    device: Arc<Mutex<Option<UiDevice>>>
}

impl Lightning {

    pub fn build(builder: &gtk::Builder) -> Lightning {

        Lightning {
            inner: builder.get_object("lightning_box").expect("could not get lightning_box"),
            combo_box: builder.get_object("lightning_input").expect("could not get lightning_input"),
            apply: builder.get_object("lightning_apply").expect("could not get lightning_apply"),
            commands: Arc::new(Mutex::new(None)),
            device: Arc::new(Mutex::new(None))
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_commands(&self, commands: (CommandFn, Vec<(String, CommandData)>), device: UiDevice) {

        self.combo_box.remove_all();
        for (name, _) in &commands.1 {
            self.combo_box.append(None, name.as_str());
        }
        self.combo_box.set_active(Some(0));

        utils::safe_lock(&self.device, move |device_lock| {
            device_lock.replace(device);
        });

        utils::safe_lock(&self.commands, move |commands_lock| {
            commands_lock.replace(commands);
        });

    }

    pub fn connect(&self, command_tx: mpsc::Sender<Option<Command>>) {

        let device_clone = Arc::clone(&self.device);
        let commands_clone = Arc::clone(&self.commands);
        let combo_box_clone = self.combo_box.clone();
        self.apply.connect_clicked(move |_| {

            let index = combo_box_clone.get_active();

            match index {

                Some(idx) => {

                    let device = utils::safe_lock(&device_clone, |device| {
                        device.clone()
                    });

                    let commands = utils::safe_lock(&commands_clone, |commands| {
                        commands.clone()
                    });

                    match commands {

                        Some(cmds) => {

                            match device {

                                Some(dev) => {
                                    let (ptr, data_vec) = cmds;
                                    let (_name, data) = &data_vec[idx as usize];
                                    command_tx.send(Some(Command {
                                        pointer: ptr,
                                        data: data.clone(),
                                        device: dev.inner
                                    })).unwrap();
                                },
                                None => {}

                            }

                        },
                        None => {}
                    }

                },
                None => {}

            }

        });


    }

}
