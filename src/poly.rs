#[derive(Clone, PartialEq, Debug)]
pub struct Poly {
    c: Vec<i64>,
}

impl Poly {
    pub fn new(mut c: Vec<i64>) -> Poly {
        while c.last() == Some(&0) {
            c.pop();
        }
        Poly { c }
    }

    pub fn coeff(&self, i: usize) -> i64 {
        self.c.get(i).copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.c.len()
    }

    pub fn add(&self, other: &Poly) -> Poly {
        let n = self.len().max(other.len());
        let mut out = Vec::new();
        for i in 0..n {
            out.push(self.coeff(i) + other.coeff(i));
        }
        Poly::new(out)
    }

    pub fn sub(&self, other: &Poly) -> Poly {
        let n = self.len().max(other.len());
        Poly::new((0..n).map(|i| self.coeff(i) - other.coeff(i)).collect())
    }

    pub fn show(&self) -> String {
        if self.len() == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        for i in (0..self.len()).rev() {
            let c = self.coeff(i);
            if c == 0 {
                continue;
            }
            if s.is_empty() {
                if c < 0 {
                    s.push('-');
                }
            } else if c < 0 {
                s.push_str(" - ");
            } else {
                s.push_str(" + ");
            }
            if c.abs() != 1 || i == 0 {
                s.push_str(&c.abs().to_string());
            }
            if i == 1 {
                s.push('x');
            } else if i > 1 {
                s.push('x');
                s.push_str(&format!("^{{{}}}", i));
            }
        }
        s
    }

    pub fn terms(&self) -> usize {
        self.c.iter().filter(|&&x| x != 0).count()
    }

    pub fn needs_brackets(&self) -> bool {
        match self.c.last() {
            None => false,
            Some(&top) => self.terms() > 1 || top < 0,
        }
    }
}
