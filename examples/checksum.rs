extern crate checksum;

use checksum::crc::Crc as crc;

use std::env;

fn checksum(filename: &str) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let filename = &args[1];
            checksum(filename);
        }
        _ => {
            println!("Pass filename in command line");
        }
    }
}
