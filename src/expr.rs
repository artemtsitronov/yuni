use crate::poly::Poly;

pub enum Expr {
    Leaf(Poly),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

pub fn eval(e: &Expr) -> Poly {
    match e {
        Expr::Leaf(p) => p.clone(),
        Expr::Add(a, b) => eval(a).add(&eval(b)),
        Expr::Sub(a, b) => eval(a).sub(&eval(b)),
    }
}

pub fn superscript(n: u32) -> String {
    const SUP: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    n.to_string()
        .chars()
        .map(|c| SUP[c.to_digit(10).unwrap() as usize])
        .collect()
}

pub fn show(e: &Expr) -> String {
    show_at(e, 0)
}

pub fn show_at(e: &Expr, level: usize) -> String {
    let (open, close) = if level % 2 == 0 {
        ('[', ']')
    } else {
        ('{', '}')
    };
    match e {
        Expr::Add(a, b) => format!(
            "{open}{} + {}{close}",
            show_at(a, level + 1),
            show_at(b, level + 1)
        ),
        Expr::Leaf(p) => {
            if p.needs_brackets() {
                format!("({})", p.show())
            } else {
                p.show()
            }
        }
        Expr::Sub(a, b) => format!(
            "{open}{} - {}{close}",
            show_at(a, level + 1),
            show_at(b, level + 1)
        ),
    }
}
