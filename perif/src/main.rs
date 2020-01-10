use gio::prelude::*;
use std::env::args;
use libperif::HidApi;
use std::sync::{Arc, Mutex};

mod utils;
mod ui;
mod consts;
mod ui_device;
mod tasks;
mod macros;

use ui::Ui;
use ui_device::UiDevice;
use utils::TaskHandler;

fn main() {

    // Load resources
    utils::load_resources();

    // HidApi mutex
    let hidapi: Arc<Mutex<HidApi>> = Arc::new(Mutex::new(HidApi::new().unwrap()));

    // Task handler
    let task_handler = Arc::new(Mutex::new(TaskHandler::new()));

    // Create app
    let task_handler_clone = Arc::clone(&task_handler);
    let application = gtk::Application::new(Some("net.olback.Perif"), Default::default()).unwrap();
    application.connect_activate(move |app| {

        // Load settings
        let settings = gio::Settings::new("net.olback.perif");

        // Build UI
        let mut ui = Ui::build(app, settings);
        ui.stack.show_no_devices(false);

        let hidapi_clone = hidapi.clone();
        let device_tx = ui.devices_view.get_devices_tx();
        let result_tx = ui.devices_view.get_error_tx();
        let command_tx = utils::safe_lock(&task_handler_clone, move |handler| {

            let (command_handle, command_tx) = tasks::command_handler(&hidapi_clone, result_tx);

            handler.add(command_handle);
            handler.add(tasks::refresh_devices(handler.get_bool(), &hidapi_clone, device_tx, 1));

            command_tx

        });

        ui.devices_view.connect_lightning(command_tx.clone());
        ui.devices_view.connect_sidetone(command_tx.clone());
        ui.devices_view.connect_commands(command_tx.clone());

        app.connect_shutdown(move |_| {
            command_tx.send(None).unwrap();
        });

    });

    // Run app
    application.run(&args().collect::<Vec<_>>());

    // On exit
    utils::safe_lock(&task_handler, |tasks| {
        println!("Waiting for threads...");
        tasks.stop_all();
    });

}
