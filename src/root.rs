use crate::bits::BitReader;
use crate::poly::Poly;

pub const ROOT_LEN: usize = 3;
pub const ROOT_B: u32 = 8;
pub const ROOT_HALF: i64 = 1 << (ROOT_B - 1);

pub fn read_root(reader: &mut BitReader) -> Poly {
    let mut c = Vec::new();
    for _ in 0..ROOT_LEN {
        c.push(reader.read(ROOT_B) as i64 - ROOT_HALF)
    }
    Poly::new(c)
}
