mod bits;
mod decode;
mod expr;
mod frame;
mod menu;
mod poly;
mod root;
use bits::BitReader;
use decode::{bits_to_bytes, explain};
use expr::{eval, show};
use frame::{frame, unframe};
use menu::build;
use root::*;

fn main() {
    let data = frame(b"Meet me at midnight");

    let total = data.len() * 8;
    let depth = pick_depth(total);
    let mut r = BitReader::new(data);
    let p = read_root(&mut r);
    let e = build(&p, depth, &mut r, total);
    println!("{} = {}", show(&e), eval(&e).show());

    let bits = explain(&e).unwrap();
    println!("used {} bits, decoded {} bits", r.pos, bits.len());
    match unframe(&bits_to_bytes(&bits)) {
        Some(d) => println!("message: {:?}", String::from_utf8_lossy(&d)),
        None => println!("Couldn't unframe"),
    }
}
