use std::sync::{
    Arc,
    Mutex
};
use libhc::{
    Device,
    HidApi
};

pub struct Command {
    pointer: fn(hidapi: &HidApi, device: &Device, data: Vec<u8>) -> bool,
    data: Vec<u8>
}

pub fn command_handler(hidapi: &Arc<Mutex<HidApi>>) {

    println!("Command handler not implemented!");

    // std::thread::spawn(move || {

    // });

}
