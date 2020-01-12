use crc::crc32;
use crate::{
    PerifResult,
    new_err
};
use super::{
    DS4_INPUT_LENGTH,
    DS4_OUTPUT_LENGTH,
    USB_OFFSET,
    BT_OFFSET,
    DS4_USB_INPUT_LENGTH,
    DS4_BT_INPUT_LENGTH
};

pub struct DS4Writer {
    inner: hidapi::HidDevice,
    pub is_bluetooth: bool,
    pub input_offset: usize,
    pub output_offset: usize
}

impl DS4Writer {

    pub fn write(&self, buf: &mut[u8]) -> PerifResult<()> {

        if self.is_bluetooth {

            if buf.len() == DS4_OUTPUT_LENGTH {

                buf[0] = 0xa2;
                buf[1] = 0x11;
                buf[2] = 0x80;
                buf[4] = 0x0f;

                let sum = crc32::checksum_ieee(&buf[0..75]);
                let bytes: [u8; 4] = unsafe { std::mem::transmute(sum.to_le()) };

                for i in 0..4 {
                    buf[75 + i] = bytes[i];
                }

                self.inner.write(&buf[1..])?;

            } else {

                return Err(new_err!("Invalid buf length"))

            }

        } else {

            buf[0] = 0x05;
            buf[1] = 0x07;
            self.inner.write(&mut buf[..])?;

        }

        Ok(())

    }

}

pub trait ToDS4Writer {
    fn open_ds4(&self, path: &std::ffi::CString) -> PerifResult<DS4Writer>;
}

impl ToDS4Writer for hidapi::HidApi {

    fn open_ds4(&self, path: &std::ffi::CString) -> PerifResult<DS4Writer> {

        let dev = self.open_path(path)?;

        let mut buf = [0; DS4_INPUT_LENGTH];
        let offsets = match dev.read(&mut buf[..]) {
            Ok(DS4_USB_INPUT_LENGTH) => USB_OFFSET,
            Ok(DS4_BT_INPUT_LENGTH) => BT_OFFSET,
            Ok(_) => return Err(new_err!("Could not determine connection type!")),
            Err(e) => return Err(new_err!(e))
        };

        let is_bluetooth = offsets != (0, 0);

        Ok(DS4Writer {
            inner: dev,
            input_offset: offsets.0,
            output_offset: offsets.1,
            is_bluetooth: is_bluetooth
        })

    }

}
