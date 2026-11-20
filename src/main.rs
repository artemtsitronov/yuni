const K: i64 = 6;

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

fn pick(n: i64, reader: &mut BitReader) -> Expr {
    let mut menu = rewrites(n);
    let k = bits_for(menu.len());
    let idx = reader.read(k);
    menu.remove(idx)
}

// (* _ *) //

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Div(a, b) => eval(a) / eval(b),
        Expr::Pow(a, e) => eval(a).pow(*e),
    }
}

fn rewrites(n: i64) -> Vec<Expr> {
    let num = |x: i64| Box::new(Expr::Num(x));
    let mut out = Vec::new();

    // powers
    for e in 2..=3u32 {
        for c in -10..=10i64 {
            if c.abs() >= 2 && c.pow(e) == n {
                out.push(Expr::Pow(num(c), e));
            }
        }
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

fn build(n: i64, depth: u32, reader: &mut BitReader) -> Expr {
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
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(build(eval(&a), depth - 1, reader)),
            Box::new(build(eval(&b), depth - 1, reader)),
        ),
        Expr::Div(a, b) => Expr::Div(
            Box::new(build(eval(&a), depth - 1, reader)),
            Box::new(build(eval(&b), depth - 1, reader)),
        ),
        Expr::Pow(a, e) => Expr::Pow(Box::new(build(eval(&a), depth - 1, reader)), e),
        Expr::Num(n) => Expr::Num(n),
    }
}

fn superscript(n: u32) -> String {
    const SUP: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    n.to_string()
        .chars()
        .map(|c| SUP[c.to_digit(10).unwrap() as usize])
        .collect()
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Num(n) => {
            if *n < 0 {
                format!("({})", n)
            } else {
                format!("{}", n)
            }
        }
        Expr::Sub(a, b) => format!("({} - {})", show(a), show(b)),
        Expr::Div(a, b) => format!("({} / {})", show(a), show(b)),
        Expr::Pow(a, e) => {
            if matches!(a.as_ref(), Expr::Pow(..)) {
                format!("({}){}", show(a), superscript(*e))
            } else {
                format!("{}{}", show(a), superscript(*e))
            }
        }
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
