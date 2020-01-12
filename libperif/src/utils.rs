// Stolen from https://github.com/Sapd/HeadsetControl/blob/master/src/utility.c

pub fn map(x: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {

    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min

}

pub fn buf_log(buf: &[u8]) {

    let len = buf.len();

    for i in 0..((len + 1) / 2) {

        let cols = (i * 2, (i * 2) + 1);

        if cols.1 < len {
            println!("[{}] 0x{:02x} = {}\t[{}] 0x{:02x} = {}", cols.0, &buf[cols.0], &buf[cols.0], cols.1, &buf[cols.1], &buf[cols.1]);
        } else {
            println!("[{}] 0x{:02x} = {}", cols.0, &buf[cols.0], &buf[cols.0]);
        }

    }

}
