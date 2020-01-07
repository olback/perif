use crate::{impl_from, is_debug};

pub type PerifResult<T> = std::result::Result<T, PerifError>;

#[derive(Debug)]
pub struct PerifError {
    cause: String,
    file: String,
    line: u32,
}

impl PerifError {

    pub fn new<C: Into<String>>(cause: C, file: &str, line: u32) -> PerifError {

        PerifError {
            cause: cause.into(),
            file: String::from(file),
            line: line
        }

    }

}

impl std::fmt::Display for PerifError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        if is_debug!() {
            write!(f, "{}#{}: {}", self.file, self.line, self.cause)
        } else {
            write!(f, "{}", self.cause)
        }

    }

}

impl_from!(hidapi::HidError);
