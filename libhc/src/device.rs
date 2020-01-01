use crate::capability::Capability;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub capabilities: Vec<Capability>
}

impl Device {

    pub fn dbg_log(&self) {

        println!("{:#?}", self);

    }

}
