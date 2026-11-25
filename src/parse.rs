use crate::expr::Expr;
use crate::poly::Poly;

struct Parser {
    c: Vec<char>,
    i: usize,
}

pub fn parse(s: &str) -> Option<Expr> {
    let mut p = Parser {
        c: s.chars().filter(|c| !c.is_whitespace()).collect(),
        i: 0,
    };
    let e = p.expr()?;
    if p.i == p.c.len() { Some(e) } else { None }
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.c.get(self.i).copied()
    }

    fn eat(&mut self, ch: char) -> Option<()> {
        if self.peek()? == ch {
            self.i += 1;
            Some(())
        } else {
            None
        }
    }

    fn eat_str(&mut self, s: &str) -> Option<()> {
        let start = self.i;
        for ch in s.chars() {
            if self.eat(ch).is_none() {
                self.i = start;
                return None;
            }
        }
        Some(())
    }

    fn term(&mut self) -> Option<(usize, i64)> {
        let mut num: Option<i64> = None;
        while let Some(d) = self.peek().and_then(|ch| ch.to_digit(10)) {
            num = Some(num.unwrap_or(0) * 10 + d as i64);
            self.i += 1;
        }
        let mut exp = 0;
        if self.eat('x').is_some() {
            exp = 1;
            if self.eat('^').is_some() {
                let braced = self.eat('{').is_some();
                let mut n: Option<usize> = None;
                while let Some(d) = self.peek().and_then(|ch| ch.to_digit(10)) {
                    n = Some(n.unwrap_or(0) * 10 + d as usize);
                    self.i += 1;
                    if !braced {
                        break;
                    }
                }
                if braced {
                    self.eat('}')?;
                }
                exp = n?;
            }
        } else if num.is_none() {
            return None;
        }
        Some((exp, num.unwrap_or(1)))
    }

    fn make_poly(&self, terms: &[(usize, i64)]) -> Poly {
        let top = terms.iter().map(|t| t.0).max().unwrap_or(0);
        let mut c = vec![0; top + 1];
        for &(e, k) in terms {
            c[e] += k;
        }
        Poly::new(c)
    }

    fn poly(&mut self) -> Option<Poly> {
        let mut terms = Vec::new();
        let first_neg = self.eat('-').is_some();
        let (e, c) = self.term()?;
        terms.push((e, if first_neg { -c } else { c }));
        loop {
            let sign = if self.eat('+').is_some() {
                1
            } else if self.eat('-').is_some() {
                -1
            } else {
                break;
            };
            let (e, c) = self.term()?;
            terms.push((e, sign * c));
        }
        Some(self.make_poly(&terms))
    }

    fn expr(&mut self) -> Option<Expr> {
        if self.eat_str("\\left").is_none() {
            let (e, c) = self.term()?;
            return Some(Expr::Leaf(self.make_poly(&[(e, c)])));
        }
        if self.eat('(').is_some() {
            let p = self.poly()?;
            self.eat_str("\\right)")?;
            return Some(Expr::Leaf(p));
        }
        let close = if self.eat('[').is_some() {
            "\\right]"
        } else {
            self.eat_str("\\{")?;
            "\\right\\}"
        };
        let left = self.expr()?;
        let is_add = if self.eat('+').is_some() {
            true
        } else {
            self.eat('-')?;
            false
        };
        let right = self.expr()?;
        self.eat_str(close)?;
        let (l, r) = (Box::from(left), Box::from(right));
        Some(if is_add {
            Expr::Add(l, r)
        } else {
            Expr::Sub(l, r)
        })
    }
}
