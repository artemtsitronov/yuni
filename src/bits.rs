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
            let byte = self.data.get(self.pos / 8).copied().unwrap_or(0);
            let bit = (byte >> (7 - self.pos % 8)) & 1;
            v = (v << 1) | bit as usize;
            self.pos += 1;
        }
        v
    }
}

pub fn bits_for(len: usize) -> u32 {
    len.ilog2()
}
