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

fn show(e: &Expr) -> String {
    match e {
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Num(n) => format!("{}", n),
    }
}

fn main() {
    for e in rewrites(234) {
        println!("{} = {}", show(&e), eval(&e));
    }
}
