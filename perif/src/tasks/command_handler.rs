use std::{
    thread::JoinHandle,
    sync::{
        Arc,
        Mutex,
        mpsc
    }
};
use libperif::{
    Device,
    HidApi,
    CommandData,
    CommandFn,
    new_err,
    PerifResult
};
use crate::utils;

pub enum CommandResult {
    Success(String),
    Error(String)
}

pub struct Command {
    pub pointer: CommandFn,
    pub data: CommandData,
    pub device: Device
}

impl Command {

    pub fn run(&self, hidapi: &HidApi) -> PerifResult<()> {

        let ptr = self.pointer;
        ptr(&hidapi, &self.device, self.data.clone())

    }

}

pub fn command_handler(hidapi: &Arc<Mutex<HidApi>>, result_tx: glib::Sender<CommandResult>) -> (JoinHandle<()>, mpsc::Sender<Option<Command>>) {

    let hidapi_clone = Arc::clone(&hidapi);
    let (tx, rx) = mpsc::channel::<Option<Command>>();

    (std::thread::spawn(move || {

        loop {

            match rx.recv() {

                Ok(command) => {

                    match command {

                        Some(cmd) => {

                            utils::safe_lock(&hidapi_clone, |lock| {

                                match cmd.run(&lock) {
                                    Ok(_) => result_tx.send(CommandResult::Success(String::from("Success"))).unwrap(),
                                    Err(e) => result_tx.send(CommandResult::Error(format!("{}", e))).unwrap()
                                }

                            });

                        },
                        None => break // Send `None` to end listening

                    }

                },
                Err(e) => {

                    eprintln!("{}", new_err!(e));
                    break;

                }

            }

        }

        println!("Stopping command thread...");

    }), tx)

}
