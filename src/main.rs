use std::fs::read;

struct BitReader {
    data: Vec<u8>,
    pos: usize,
}

impl BitReader {
    fn read(&mut self, k: u32) -> usize {
        let mut v = 0;
        for _ in 0..k {
            let byte = self.data.get(self.pos / 8).copied().unwrap_or(0);
            let bit = (byte >> (7 - self.pos % 8)) & 1;
            v = (v << 1) | bit as usize;
            self.pos += 1;
        }
        v
    }
}

fn bits_for(len: usize) -> u32 {
    len.ilog2()
}

fn pick(n: u32, reader: &mut BitReader) -> Expr {
    let mut menu = rewrites(n);
    let k = bits_for(menu.len());
    let idx = reader.read(k);
    menu.remove(idx)
}

// (* _ *) //

enum Expr {
    Num(u32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> u32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn rewrites(n: u32) -> Vec<Expr> {
    let mut out = Vec::new();
    for a in 1..n {
        out.push(Expr::Add(
            Box::from(Expr::Num(a)),
            Box::from(Expr::Num(n - a)),
        ))
    }
    for a in 2..n {
        if n % a == 0 {
            out.push(Expr::Mul(
                Box::from(Expr::Num(a)),
                Box::from(Expr::Num(n / a)),
            ))
        }
    }
    out
}

fn build(n: u32, depth: u32, reader: &mut BitReader) -> Expr {
    if depth == 0 || rewrites(n).is_empty() {
        return Expr::Num(n);
    }
    match pick(n, reader) {
        Expr::Add(a, b) => Expr::Add(
            Box::new(build(eval(&a), depth - 1, reader)),
            Box::new(build(eval(&b), depth - 1, reader)),
        ),
        Expr::Mul(a, b) => Expr::Mul(
            Box::new(build(eval(&a), depth - 1, reader)),
            Box::new(build(eval(&b), depth - 1, reader)),
        ),
        other => other,
    }
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Num(n) => format!("{}", n),
    }
}

fn main() {
    let mut r = BitReader {
        data: b"Hi hack club!".to_vec(),
        pos: 0,
    };
    let e = build(12, 3, &mut r);
    println!("{} = {}", show(&e), eval(&e));
}
