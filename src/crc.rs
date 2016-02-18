use crc32::Crc32 as crc32;
use crc64::Crc64 as crc64;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub struct Checksum {
    pub crc32: u32,
    pub crc64: u64,
}

pub struct Crc {
    checksum: Checksum,
    path: String,
}

impl Crc {
    pub fn new(filepath: &str) -> Crc {
        let crc = Crc {
            checksum: Checksum {
                crc32: 0x00000000,
                crc64: 0x0000000000000000,
            },
            path: filepath.to_string(),
        };
        crc
    }

    pub fn checksum(&mut self) -> Result<&Checksum, &str> {
        let path = Path::new(&self.path);
        let mut buffer = [0u8; 1_048_576];
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                let res: Result<&Checksum, &str> = Err("File Open Error");
                return res;
            }
        };

        let mut c32 = crc32::new();
        let mut c64 = crc64::new();
        while match file.read(&mut buffer) {
            Ok(len) => {
                c32.update(&buffer[0..len]);
                c64.update(&buffer[0..len]);
                len > 0
            }
            Err(_) => {
                let res: Result<&Checksum, &str> = Err("File Read Error");
                return res;
            }
        } {}
        c32.finalize();
        c64.finalize();

        self.checksum.crc32 = c32.getsum();
        self.checksum.crc64 = c64.getsum();
        let res: Result<&Checksum, &str> = Ok(&self.checksum);
        return res;
    }

    pub fn print_checksum(&self) {
        println!("{:X}", self.checksum.crc32);
        println!("{:X}", self.checksum.crc64);
    }

    pub fn getsums(&self) -> &Checksum {
        &self.checksum
    }
}

#[test]
fn crc_test() {
    use std::fs::File;
    use std::fs;

    let file = File::create("foo.txt").unwrap();
    file.set_len(1572864).unwrap();

    let mut crc = Crc::new("foo.txt");
    crc.checksum().unwrap();
    let sums: &Checksum = crc.getsums();

    fs::remove_file("foo.txt").unwrap();

    assert_eq!(sums.crc32,0xAC2C5EE1);
    assert_eq!(sums.crc64,0xD39C35166B57EF79);
}
