pub struct Crc64 {
    table: [u64; 256],
    value: u64,
    count: u32,
}

fn table_maker(polynomial: u64) -> [u64; 256] {
    let mut table: [u64; 256] = [0; 256];
    for i in 0..256 {
        let mut v = i as u64;
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

impl Crc64 {
    pub fn new() -> Crc64 {
        let polynomial = 0xC96C5795D7870F42;
        let c64 = Crc64 {
            table: table_maker(polynomial),
            value: 0xffffffffffffffff,
            count: 0,
        };
        c64
    }

    pub fn reset(&mut self) {
        self.value = 0xffffffffffffffff;
        self.count = 0;
    }

    pub fn update(&mut self, buf: &[u8]) {
        for &i in buf {
            self.value = self.table[((self.value as u8) ^ i) as usize] ^ (self.value >> 8);
            self.count += 1;
        }
    }

    pub fn finalize(&mut self) {
        self.value = self.value ^ 0xffffffffffffffffu64;
    }

    pub fn checksum(&mut self, buf: &[u8]) -> u64 {
        self.reset();
        self.update(buf);
        self.finalize();
        self.getsum()
    }

    pub fn getsum(&self) -> u64 {
        self.value
    }

    pub fn bytecount(&self) -> u32 {
        self.count
    }
}

#[test]
fn crc32_test() {
    let mut crc64 = Crc64::new();
    crc64.checksum(b"0000");
    assert_eq!(crc64.getsum(), 0x2B5C866A7CBEF446);
}
