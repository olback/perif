#[macro_export]
macro_rules! get_obj {
    ($builder:expr, $id:expr) => {
        // Catch and panic manually to get useful file and line info
        match $builder.get_object($id) {
            Some(o) => o,
            None => panic!("could not get {}", $id)
        }
    };
}
