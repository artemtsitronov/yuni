use crate::bits::BitReader;
use crate::expr::Expr;
use crate::poly::Poly;

pub const B: u32 = 7;
pub const HALF: i64 = 1 << (B - 1);
pub const MINLEN: usize = 2;

fn make(p: &Poly, reader: &mut BitReader) -> Expr {
    let fam = reader.read(1);
    let l = p.len().max(MINLEN);
    let mut a = Vec::new();
    for _ in 0..l {
        a.push(reader.read(B) as i64 - HALF);
    }
    let a = Poly::new(a);
    if fam == 0 {
        Expr::Add(
            Box::from(Expr::Leaf(a.clone())),
            Box::from(Expr::Leaf(p.sub(&a))),
        )
    } else {
        Expr::Sub(Box::from(Expr::Leaf(p.add(&a))), Box::from(Expr::Leaf(a)))
    }
}

pub fn build(p: &Poly, depth: u32, reader: &mut BitReader) -> Expr {
    if depth == 0 {
        return Expr::Leaf(p.clone());
    }
    let e = make(p, reader);
    grow(e, depth - 1, reader)
}

fn grow(e: Expr, depth: u32, reader: &mut BitReader) -> Expr {
    match e {
        Expr::Add(a, b) => Expr::Add(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Leaf(p) => build(&p, depth, reader),
    }
}
