use crate::bits::bits_for;
use crate::expr::{Expr, eval};
use crate::menu::{B, HALF, MINLEN};
use crate::poly::Poly;

pub fn explain_build(p: &Poly, depth: u32, e: &Expr) -> Option<Vec<bool>> {
    if eval(e) != *p {
        return None;
    }
    if depth == 0 {
        return if matches!(e, Expr::Leaf(_)) {
            Some(Vec::new())
        } else {
            None
        };
    }
    let l = p.len().max(MINLEN);

    let (fam, a, left, right, lval, rval) = match e {
        Expr::Add(x, y) => {
            let a = eval(x);
            let b = p.sub(&a);
            (0, a.clone(), x, y, a, b)
        }
        Expr::Sub(x, y) => {
            let a = eval(y);
            let b = p.add(&a);
            (1, a.clone(), x, y, b, a)
        }
        _ => return None,
    };
    if a.len() > l {
        return None;
    }
    let mut bits = idx_to_bits(fam, 1);
    for i in 0..l {
        let v = a.coeff(i) + HALF;
        if v < 0 || v >= (1 << B) {
            return None;
        }
        bits.extend(idx_to_bits(v as usize, B));
    }
    bits.extend(explain_build(&lval, depth - 1, left)?);
    bits.extend(explain_build(&rval, depth - 1, right)?);
    Some(bits)
}

fn explain_grow(shape: &Expr, e: &Expr, depth: u32) -> Option<Vec<bool>> {
    match (shape, e) {
        (Expr::Leaf(p), _) => explain_build(&p.clone(), depth, e),
        (Expr::Add(a1, b1), Expr::Add(a2, b2))
        | (Expr::Sub(a1, b1), Expr::Sub(a2, b2))
        | (Expr::Mul(a1, b1), Expr::Mul(a2, b2)) => {
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
