pub struct Crc32 {
    table: [u32; 256],
    value: u32,
    count: u32,
}

fn table_maker(polynomial: u32) -> [u32; 256] {
    let mut table: [u32; 256] = [0; 256];
    for i in 0..256 {
        let mut v = i as u32;
        for _ in 0..8 {
            v = if v & 1 != 0 {
                polynomial ^ (v >> 1)
            } else {
                v >> 1
            }
        }
        table[i] = v;
    }
    table
}

impl Crc32 {
    pub fn new() -> Crc32 {
        let polynomial = 0xEDB88320;
        let c32 = Crc32 {
            table: table_maker(polynomial),
            value: 0xffffffff,
            count: 0,
        };
        c32
    }

    pub fn reset(&mut self) {
        self.value = 0xffffffff;
        self.count = 0;
    }

    pub fn update(&mut self, buf: &[u8]) {
        for &i in buf {
            self.value = self.table[((self.value as u8) ^ i) as usize] ^ (self.value >> 8);
            self.count += 1;
        }
    }

    pub fn finalize(&mut self) {
        self.value = self.value ^ 0xffffffff;
    }

    pub fn checksum(&mut self, buf: &[u8]) -> u32 {
        self.reset();
        self.update(buf);
        self.finalize();
        self.getsum()
    }

    pub fn getsum(&self) -> u32 {
        self.value
    }

    pub fn bytecount(&self) -> u32 {
        self.count
    }
}

#[test]
fn crc32_test() {
    let mut crc32 = Crc32::new();
    crc32.checksum(b"0000");
    assert_eq!(crc32.getsum(), 0x0C9BC472);
}
