use core::hash::Hasher;

pub struct DJB2(u64);

impl Hasher for DJB2 {
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 = ((self.0 << 5) + self.0) + (*b as u64); // hash * 33 + bytes[i]
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

impl DJB2 {
    pub fn new() -> Self {
        DJB2(5381)
    }
}
