use libperif;
use std::{fs, path::PathBuf};

const PRIORITY: u8 = 52;
const OUT_PATH: &'static str = "udev/rules.d";

fn main() {

    println!("Generating udev rules...\n");

    let path = PathBuf::from(OUT_PATH);

    // Delete old rules
    for file in fs::read_dir(&path).unwrap() {
        fs::remove_file(file.unwrap().path()).unwrap();
    }

    let devices = libperif::get_supported_devices();
    for device in devices {

        let device_name = device.name.to_lowercase().replace(" ", "-").replace("(", "").replace(")", "");
        let file_name = format!("{}-{}-{:04x}.rules", PRIORITY, device_name, device.pid);
        let file_content = format!("SUBSYSTEM==\"hidraw\", ATTRS{{idVendor}}==\"{vid:04x}\", ATTRS{{idProduct}}==\"{pid:04x}\", MODE=\"0666\"\n", vid = device.vid, pid = device.pid);

        println!("{}\n{}", file_name, file_content);
        fs::write(path.as_path().join(file_name), file_content).unwrap();

    }

    println!("Done!");

}
