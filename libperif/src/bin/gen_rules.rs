use libperif;
use std::fs;

fn main() {

    println!("Generating udev rules...");


    let mut lines = Vec::<String>::new();
    let devices = libperif::get_supported_devices();

    for device in devices {
        lines.push(format!("SUBSYSTEM==\"hidraw\", ATTRS{{idVendor}}==\"{vid:04x}\", ATTRS{{idProduct}}==\"{pid:04x}\", MODE=\"0666\"", vid = device.vid, pid = device.pid));
    }

    lines.push(String::new());

    let file_content = lines.join("\n");

    println!("{}", file_content);

    fs::write("udev/52-perif.rules", file_content).unwrap();

    println!("Done!");

}
