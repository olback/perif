use libperif::{self, HidApi};

fn main() {

    let mut hidapi = HidApi::new().unwrap();
    hidapi.refresh_devices().unwrap();

    println!("<manufacturer> <product_string> <vid>:<pid>");

    for dev in hidapi.device_list() {

        let mfr = dev.manufacturer_string().clone().unwrap_or("unknown");
        let prd = dev.product_string().clone().unwrap_or("unknown");

        println!("{} {} {:04x}:{:04x}", mfr, prd, dev.vendor_id(), dev.product_id());

    }

}
