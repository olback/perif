use gtk::prelude::*;
use libperif::CommandFn;
use std::sync::{
    Arc,
    Mutex,
    mpsc
};
use crate::{
    utils,
    tasks::Command,
    ui_device::UiDevice,
    get_obj
};

#[derive(Clone)]
pub struct Sidetone {
    inner: gtk::Box,
    scale: gtk::Scale,
    apply: gtk::Button,
    command: Arc<Mutex<Option<CommandFn>>>,
    device: Arc<Mutex<Option<UiDevice>>>
}

impl Sidetone {

    pub fn build(builder: &gtk::Builder) -> Sidetone {

        Sidetone {
            inner: get_obj!(builder, "sidetone_box"),
            scale: get_obj!(builder, "sidetone_scale"),
            apply: get_obj!(builder, "sidetone_apply"),
            command: Arc::new(Mutex::new(None)),
            device: Arc::new(Mutex::new(None))
        }

    }

    pub fn set_visible(&self, visible: bool) {

        self.inner.set_visible(visible);

    }

    pub fn set_command(&self, command: CommandFn, device: UiDevice) {

        utils::safe_lock(&self.device, move |device_lock| {
            device_lock.replace(device);
        });

        utils::safe_lock(&self.command, move |command_lock| {
            command_lock.replace(command);
        });

    }

    pub fn connect(&self, command_tx: mpsc::Sender<Option<Command>>) {


        let device_clone = Arc::clone(&self.device);
        let command_clone = Arc::clone(&self.command);
        let scale_clone = self.scale.clone();
        self.apply.connect_clicked(move |_| {

            let device = utils::safe_lock(&device_clone, move |device_lock| {
                device_lock.clone()
            });

            let command = utils::safe_lock(&command_clone, move |command_lock| {
                command_lock.clone()
            });

            match command {

                Some(cmd) => {

                    match device {

                        Some(dev) => {

                            let raw_value = scale_clone.get_value() as i32;
                            let value = libperif::utils::map(raw_value, 0, 100, 0, 255) as u8;

                            command_tx.send(Some(Command {
                                pointer: cmd,
                                data: vec![value],
                                device: dev.inner
                            })).unwrap();

                        },
                        None => {}

                    }

                },
                None => {}

            }

        });


    }

}
