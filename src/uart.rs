use std::fs::{self, File, OpenOptions};

pub struct MyUart {
    pub device: File,
    pub baudrate: u32,
    pub parity: u8,
    pub stop_bits: u8,
    pub data_bits: u8,

}

impl MyUart {
    pub fn new(baudrate: u32, parity: u8, stop_bits: u8, data_bits: u8) -> MyUart {
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/ttyAMA0")
            .unwrap();
        MyUart {
            device,
            baudrate,
            parity,
            stop_bits,
            data_bits,
        }
    }
}