mod device;
mod capability;

#[cfg(test)]
mod tests {
    use super::{
        device::Device,
        capability::Capability
    };
    #[test]
    fn test_code() {

        let d = Device {
            name: String::from("Corsair Void (Pro)"),
            capabilities: vec![
                Capability::BATTERY,
                Capability::LIGHTNING,
                Capability::SIDETONE,
                Capability::NOTIFICATION_SOUND
            ]
        };

        d.dbg_log();

        assert!(true);

    }
}
