mod bits;
mod cli;
mod decode;
mod expr;
mod frame;
mod menu;
mod poly;
mod root;

use bits::BitReader;
use cli::{Cli, Cmd};
use decode::{bits_to_bytes, explain};
use expr::{Expr, eval, show};
use frame::{frame, unframe};
use menu::build;
use root::*;

fn encode(msg: &[u8]) -> Expr {
    let framed = frame(msg);
    let total = framed.len() * 8;
    let depth = pick_depth(total);
    let mut r = BitReader::new(framed);
    let p = read_root(&mut r);
    build(&p, depth, &mut r, total)
}

fn decode(e: &Expr) -> Option<String> {
    let bits = explain(e)?;
    let bytes = unframe(&bits_to_bytes(&bits))?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Encode { message } => {
            let e = encode(message.as_bytes());
            println!("{} = {}", show(&e), eval(&e).show());
        }
        Cmd::Decode { .. } => todo!(),
    }
}
