use gio::prelude::*;
use std::env::args;
use libhc::HidApi;
use std::sync::{Arc, Mutex};

mod utils;
mod ui;
mod consts;
mod ui_device;
mod tasks;

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
    let application = gtk::Application::new(Some("net.olback.headset-control"), Default::default()).unwrap();
    application.connect_activate(move |app| {

        // Build UI
        let ui = Ui::build(app);
        ui.stack.show_no_devices(false);

        let hidapi_clone = hidapi.clone();
        let device_tx = ui.devices_view.get_tx();
        let command_tx = utils::safe_lock(&task_handler_clone, move |handler| {

            let (command_handle, command_tx) = tasks::command_handler(&hidapi_clone);

            handler.add(command_handle);
            handler.add(tasks::refresh_devices(handler.get_bool(), &hidapi_clone, device_tx, 1));

            command_tx

        });

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
