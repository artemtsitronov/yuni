use crate::bits::bits_for;
use crate::expr::{Expr, eval};
use crate::menu::rewrites;

pub fn explain_build(n: i64, depth: u32, e: &Expr) -> Option<Vec<bool>> {
    if eval(e) != n {
        return None;
    }
    let menu = rewrites(n);
    if depth == 0 || menu.is_empty() {
        return if matches!(e, Expr::Num(_)) {
            Some(Vec::new())
        } else {
            None
        };
    }
    let k = bits_for(menu.len());
    for idx in 0..(1usize << k) {
        if let Some(rest) = explain_grow(&menu[idx], e, depth - 1) {
            let mut bits = idx_to_bits(idx, k);
            bits.extend(rest);
            return Some(bits);
        }
    }
    None
}

fn explain_grow(shape: &Expr, e: &Expr, depth: u32) -> Option<Vec<bool>> {
    match (shape, e) {
        (Expr::Num(n), _) => explain_build(*n, depth, e),
        (Expr::Add(a1, b1), Expr::Add(a2, b2))
        | (Expr::Sub(a1, b1), Expr::Sub(a2, b2))
        | (Expr::Mul(a1, b1), Expr::Mul(a2, b2))
        | (Expr::Div(a1, b1), Expr::Div(a2, b2)) => {
            let mut bits = explain_grow(&a1, a2, depth)?;
            bits.extend(explain_grow(&b1, b2, depth)?);
            Some(bits)
        }
        (Expr::Pow(a1, e1), Expr::Pow(a2, e2)) if e1 == e2 => explain_grow(&a1, a2, depth),
        _ => None,
    }
}

pub fn idx_to_bits(idx: usize, k: u32) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in (0..k).rev() {
        bits.push((idx >> i) & 1 == 1);
    }
    bits
}

pub fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |= 1 << (7 - i);
            }
        }
        bytes.push(byte)
    }
    bytes
}
