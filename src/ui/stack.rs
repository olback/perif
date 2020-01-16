use gtk::prelude::*;
use crate::{
    ui_device::UiDevice,
    utils,
    get_obj
};
use std::sync::{
    Arc,
    Mutex
};

#[derive(Clone)]
pub struct Stack {
    inner: gtk::Stack,
    transition: gtk::StackTransitionType,
    devices: Arc<Mutex<Vec<UiDevice>>>
}

impl Stack {

    pub fn build(builder: &gtk::Builder, devices: &Arc<Mutex<Vec<UiDevice>>>) -> Stack {

        let stack: gtk::Stack = get_obj!(builder, "views");

        Stack {
            transition: stack.get_transition_type(),
            inner: stack,
            devices: Arc::clone(&devices)
        }

    }

    pub fn show_devices(&self, transition: bool) {

        self.enable_transition(transition);

        let len = utils::safe_lock(&self.devices, |devices| {
            devices.len()
        });

        if len > 0 {
            self.inner.set_visible_child_name("devices");
        } else {
            self.show_no_devices(transition);
        }

    }

    pub fn show_no_devices(&self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("no_devices");

    }

    pub fn show_settings(&mut self, transition: bool) {

        self.enable_transition(transition);
        self.inner.set_visible_child_name("settings");

    }

    pub fn hide_settings(&self, transition: bool) {

        self.show_devices(transition);

    }

    pub fn get_view_name(&self) -> String {

        self.inner.get_visible_child_name().unwrap().to_string()

    }

    fn enable_transition(&self, enable: bool) {

        if enable {
            self.inner.set_transition_type(self.transition);
        } else {
            self.inner.set_transition_type(gtk::StackTransitionType::None);
        }

    }

}
