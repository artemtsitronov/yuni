use crate::expr::{Expr, eval};
use crate::menu::{B, HALF, MINLEN};
use crate::poly::Poly;
use crate::root::{ROOT_B, ROOT_HALF, ROOT_LEN};

pub fn explain_build(p: &Poly, e: &Expr) -> Option<Vec<bool>> {
    if eval(e) != *p {
        return None;
    }
    let l = p.len().max(MINLEN);

    let (fam, a, left, right, lval, rval) = match e {
        Expr::Leaf(_) => return Some(Vec::new()),
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
        if !(0..(1 << B)).contains(&v) {
            return None;
        }
        bits.extend(idx_to_bits(v as usize, B));
    }
    bits.extend(explain_build(&lval, left)?);
    bits.extend(explain_build(&rval, right)?);
    Some(bits)
}

pub fn root_bits(p: &Poly) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..ROOT_LEN {
        bits.extend(idx_to_bits((p.coeff(i) + ROOT_HALF) as usize, ROOT_B));
    }
    bits
}

pub fn explain(e: &Expr) -> Option<Vec<bool>> {
    let p = eval(e);
    let mut bits = root_bits(&p);
    bits.extend(explain_build(&p, e)?);
    Some(bits)
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
