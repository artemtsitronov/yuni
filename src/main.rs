mod bits;
mod cli;
mod decode;
mod expr;
mod frame;
mod menu;
mod parse;
mod poly;
mod root;

use bits::BitReader;
use clap::Parser;
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

fn decode(line: &str) -> Option<String> {
    let (left, _) = line.split_once('=')?;
    let e = parse::parse(left)?;
    let bits = explain(&e)?;
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
        Cmd::Decode { expr } => match decode(&expr) {
            Some(msg) => println!("{msg}"),
            None => println!("OH NO! I COULDN'T DECODE AAAH ( please forgive me )"),
        },
    }
}
