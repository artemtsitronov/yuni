use crate::bits::{BitReader, bits_for};
use crate::expr::Expr;

const K: i64 = 6;

fn pick(n: i64, reader: &mut BitReader) -> Expr {
    let mut menu = rewrites(n);
    let k = bits_for(menu.len());
    let idx = reader.read(k);
    menu.remove(idx)
}

pub fn rewrites(n: i64) -> Vec<Expr> {
    let num = |x: i64| Box::from(Expr::Num(x));
    let mut out = Vec::new();

    // powers
    for e in 2..=3u32 {
        for c in -10..=10i64 {
            if c.abs() >= 2 && c.pow(e) == n {
                out.push(Expr::Pow(num(c), e));
            }
        }
    }

    // squares: n = c² + (n-c²) and n = c² - (c² - n)
    for c in 2..=4i64 {
        let sq = c * c;
        out.push(Expr::Add(Box::from(Expr::Pow(num(c), 2)), num(n - sq)));
        out.push(Expr::Sub(Box::from(Expr::Pow(num(c), 2)), num(sq - n)));
    }

    // division: n = (n*k) / k
    for k in 2..=K {
        out.push(Expr::Div(num(n * k), num(k)));
    }

    // substraction: n = (n+k) - k
    for k in 1..=K {
        out.push(Expr::Sub(num(n + k), num(k)));
    }

    // adding a negative num: n = (n+k) + (-k)
    for k in 1..=K {
        out.push(Expr::Add(num(n + k), num(-k)));
    }

    // multiplication: n = a * (n/a)
    for a in 2..=K {
        if n % a == 0 {
            out.push(Expr::Mul(num(a), num(n / a)));
        }
    }

    // addition: n = a + (n-a)
    for a in 1..n {
        out.push(Expr::Add(num(a), num(n - a)));
    }

    out
}

pub fn build(n: i64, depth: u32, reader: &mut BitReader) -> Expr {
    if depth == 0 || rewrites(n).is_empty() {
        return Expr::Num(n);
    }
    let e = pick(n, reader);
    grow(e, depth - 1, reader)
}

fn grow(e: Expr, depth: u32, reader: &mut BitReader) -> Expr {
    match e {
        Expr::Add(a, b) => Expr::Add(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Mul(a, b) => Expr::Mul(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Div(a, b) => Expr::Div(
            Box::new(grow(*a, depth, reader)),
            Box::new(grow(*b, depth, reader)),
        ),
        Expr::Pow(a, e) => Expr::Pow(Box::new(grow(*a, depth, reader)), e),
        Expr::Num(n) => build(n, depth, reader),
    }
}
