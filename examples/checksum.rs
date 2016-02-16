extern crate checksum;

use checksum::crc::Crc as crc;

use std::env;

pub fn main() {
    let filename: &str = &env::args().nth(2).unwrap()[..];
    let mut crc = crc::new(filename);
    match crc.checksum() {
        Ok(checksum) => {
            println!("CRC32: {:X}", checksum.crc32);
            println!("CRC64: {:X}", checksum.crc64);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
