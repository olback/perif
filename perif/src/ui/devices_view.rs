use gtk::prelude::*;
use libperif::DeviceKind;
use crate::{
    ui_device::UiDevice,
    utils,
    tasks::CommandResult
};
use std::sync::{
    Arc,
    Mutex
};
use super::{
    stack::Stack,
    device_view::DeviceView
};

pub struct DevicesView {
    devices: Arc<Mutex<Vec<UiDevice>>>,
    device_list: gtk::ListBox,
    device_view: DeviceView,
    tx: glib::Sender<Vec<UiDevice>>
}

impl DevicesView {

    pub fn build(builder: &gtk::Builder, stack: &Stack, devices: &Arc<Mutex<Vec<UiDevice>>>) -> DevicesView {

        let (tx, rx) = glib::MainContext::channel::<Vec<UiDevice>>(glib::PRIORITY_DEFAULT);

        let inner = DevicesView {
            devices: Arc::clone(&devices),
            device_list: builder.get_object("device_list").expect("could not get device_list"),
            device_view: DeviceView::build(&builder),
            tx
        };

        Self::clear_device_list(&inner.device_list);

        let devices_clone = Arc::clone(&inner.devices);
        let device_view_clone = inner.device_view.clone();
        inner.device_list.connect_row_activated(move |_, row| {

            let index = row.get_index();
            let ui_devs = utils::safe_lock(&devices_clone, |devs_lock| {
                devs_lock.clone()
            });

            device_view_clone.show_device(ui_devs[index as usize].clone());

        });

        let devices_clone = Arc::clone(&inner.devices);
        let device_list_clone = inner.device_list.clone();
        let device_view_clone = inner.device_view.clone();
        let stack_clone = stack.clone();
        rx.attach(None, move |devices| {

            let len = utils::safe_lock(&devices_clone, |devices_lock| {
                devices_lock.len()
            });

            if devices.len() > 0 && stack_clone.get_view_name() == "no_devices" {
                stack_clone.show_devices(true);
            } else if devices.len() == 0 && stack_clone.get_view_name() == "devices" {
                stack_clone.show_no_devices(true);
            }

            let selected_rows = device_list_clone.get_selected_rows();

            if devices.len() > 0 && selected_rows.len() == 0 {
                device_view_clone.show_device(devices[0].clone());
            }

            if selected_rows.len() > 0 {
                let index = selected_rows[0].get_index() as usize;
                if index < devices.len() {
                    device_view_clone.show_device(devices[index].clone());
                }
            }

            if len != devices.len() {

                Self::clear_device_list(&device_list_clone);

                for dev in &devices {
                    Self::list_add_device(&device_list_clone, &dev);
                }

            }

            // Cannot use utils::safe_lock here since we can't assign to an argument
            let mut devices_lock = devices_clone.lock().unwrap();
            *devices_lock = devices;
            drop(devices_lock);

            glib::Continue(true)

        });

        inner

    }

    fn clear_device_list(device_list: &gtk::ListBox) {

        for child in device_list.get_children() {
            device_list.remove(&child);
        }

    }

    fn list_add_device(device_list: &gtk::ListBox, device: &UiDevice) {

        let list_box_row = gtk::ListBoxRow::new();

        let icon_name = match device.inner.kind {
            DeviceKind::Bluetooth => "bluetooth-symbolic",
            DeviceKind::Controller => "input-gaming-symbolic",
            DeviceKind::Headphones => "audio-headphones-symbolic",
            DeviceKind::Headset => "audio-headset-symbolic",
            DeviceKind::Keyboard => "input-keyboard-symbolic",
            DeviceKind::Mouse => "input-mouse-symbolic",
            DeviceKind::Tablet => "input-tablet-symbolic"
        };

        let icon = gtk::Image::new_from_icon_name(Some(icon_name), gtk::IconSize::LargeToolbar);
        let label = gtk::Label::new(Some(device.inner.name.as_str()));

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.set_spacing(6);
        container.set_margin_start(10);
        container.set_margin_end(12);
        container.pack_start(&icon, false, true, 0);
        container.pack_start(&label, false, true, 0);

        list_box_row.add(&container);

        device_list.insert(&list_box_row, -1);
        device_list.show_all();

    }

    pub fn get_devices_tx(&self) -> glib::Sender<Vec<UiDevice>> {
        self.tx.clone()
    }

    pub fn get_error_tx(&self) -> glib::Sender<CommandResult> {
        self.device_view.get_tx()
    }

}
