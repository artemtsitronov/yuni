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

pub fn show(e: &Expr) -> String {
    show_at(e, 0)
}

pub fn show_at(e: &Expr, level: usize) -> String {
    let (open, close) = if level % 2 == 0 {
        ("\\left[", "\\right]")
    } else {
        ("\\left\\{", "\\right\\}")
    };
    match e {
        Expr::Add(a, b) => format!(
            "{open}{} + {}{close}",
            show_at(a, level + 1),
            show_at(b, level + 1)
        ),
        Expr::Leaf(p) => {
            if p.needs_brackets() {
                format!("\\left({}\\right)", p.show())
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
