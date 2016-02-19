# checksum-rs
Calculates CRC32/CRC64 file checksums written in Rust. 
CRC32 uses the IEEE polynomial 0xEDB88320 by default and 
CRC64 uses the ECMA polynomial 0xC96C5795D7870F42
#### Usage
```rust
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
```
#### License
MIT
