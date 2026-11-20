mod bits;
mod expr;
mod menu;
use bits::BitReader;
use expr::{eval, show};
use menu::build;

fn main() {
    let mut r = BitReader::new(b"Hi hack club!".to_vec());
    let e = build(12, 3, &mut r);
    println!("{} = {}", show(&e), eval(&e));
}
