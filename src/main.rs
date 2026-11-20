mod bits;
mod decode;
mod expr;
mod menu;
mod poly;
use bits::BitReader;
use decode::{bits_to_bytes, explain_build};
use expr::{eval, show};
use menu::build;
use poly::Poly;

const DEPTH: u32 = 4;

fn main() {
    let data = b"Hi!".to_vec();

    let mut r = BitReader::new(data);
    let p = Poly::constant(8);
    let e = build(&p, DEPTH, &mut r);
    println!("{} = {}", show(&e), eval(&e).show());

    let bits = explain_build(&p, DEPTH, &e).unwrap();
    println!("used {} bits, decoded {} bits", r.pos, bits.len());
    println!("{}", String::from_utf8_lossy(&bits_to_bytes(&bits)));
}
