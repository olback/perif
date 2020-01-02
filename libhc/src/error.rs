use crate::{impl_from, is_debug};

pub type HCResult<T> = std::result::Result<T, HCError>;

#[derive(Debug)]
pub struct HCError {
    cause: String,
    file: String,
    line: u32,
}

impl HCError {

    pub fn new<C: Into<String>>(cause: C, file: &str, line: u32) -> HCError {

        HCError {
            cause: cause.into(),
            file: String::from(file),
            line: line
        }

    }

}

impl std::fmt::Display for HCError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        if is_debug!() {
            write!(f, "{}#{}: {}", self.file, self.line, self.cause)
        } else {
            write!(f, "{}", self.cause)
        }

    }

}

impl_from!(hidapi::HidError);
