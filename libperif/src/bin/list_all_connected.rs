use libperif::{self, HidApi};

fn main() {

    let mut hidapi = HidApi::new().unwrap();
    hidapi.refresh_devices().unwrap();

    println!("<manufacturer> <product_string> <vid>:<pid>");

    for dev in hidapi.devices() {

        let mfr = dev.manufacturer_string.clone().unwrap_or(String::from("unknown"));
        let prd = dev.product_string.clone().unwrap_or(String::from("unknown"));

        println!("{} {} {:04x}:{:04x}", mfr, prd, dev.vendor_id, dev.product_id);

    }

}
