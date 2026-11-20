mod bits;
mod decode;
mod expr;
mod menu;
use bits::BitReader;
use decode::{bits_to_bytes, explain_build};
use expr::{eval, show};
use menu::build;

fn main() {
    let data = b"Hi!".to_vec();

    let mut r = BitReader::new(data);
    let e = build(12, 3, &mut r);
    println!("{} = {}", show(&e), eval(&e));

    let bits = explain_build(eval(&e), 3, &e).unwrap();
    println!("used {} bits, decoded {} bits", r.pos, bits.len());
    println!("{}", String::from_utf8_lossy(&bits_to_bytes(&bits)));
}
