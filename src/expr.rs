pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
}

pub fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Div(a, b) => eval(a) / eval(b),
        Expr::Pow(a, e) => eval(a).pow(*e),
    }
}

fn superscript(n: u32) -> String {
    const SUP: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    n.to_string()
        .chars()
        .map(|c| SUP[c.to_digit(10).unwrap() as usize])
        .collect()
}

pub fn show(e: &Expr) -> String {
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
