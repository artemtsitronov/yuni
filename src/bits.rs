use rand::random;

pub struct BitReader {
    data: Vec<u8>,
    pub pos: usize,
}

impl BitReader {
    pub fn new(data: Vec<u8>) -> BitReader {
        BitReader { data, pos: 0 }
    }

    pub fn read(&mut self, k: u32) -> usize {
        let mut v = 0;
        for _ in 0..k {
            let bit = if self.pos / 8 < self.data.len() {
                let byte = self.data[self.pos / 8];
                (byte >> (7 - self.pos % 8)) & 1
            } else {
                random::<bool>() as u8
            };
            v = (v << 1) | bit as usize;
            self.pos += 1;
        }
        v
    }
}
