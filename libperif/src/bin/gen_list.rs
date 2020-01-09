use libperif;
use std::{fs, path::PathBuf};

const OUT_PATH: &'static str = "supported_devices.txt";

fn main() {

    println!("Generating list of supported devices...\n");

    let path = PathBuf::from(OUT_PATH);
    let mut lines = Vec::<String>::new();
    let devices = libperif::get_supported_devices();

    for device in devices {

        let line = format!("{} {:04x}:{:04x}", device.name, device.vid, device.pid);
        println!("{}", line);
        lines.push(line);

    }

    lines.push(String::from(""));

    let file_content = lines.join("\n");

    fs::write(path, file_content).unwrap();

    println!("\nDone!");

}
