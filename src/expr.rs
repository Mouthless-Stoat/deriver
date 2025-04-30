use std::f64::consts::E;

mod combinator;
mod derive;
mod display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    Sin,
    Cos,
    Tan,

    Csc,
    Sec,
    Cot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bin {
    Add,
    Sub,
    Div,
    Mul,
    /// 0 is base, 1 is power
    Exp,
    /// 0 is base, 1 is augument
    Log,
}

impl Bin {
    pub fn precedence(&self) -> usize {
        match self {
            Bin::Add | Bin::Sub => 1,
            Bin::Div | Bin::Mul => 2,
            Bin::Exp | Bin::Log => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Bin(Bin, Box<Expr>, Box<Expr>),
    Trig(Trig, Box<Expr>),
    Var,
    Num(f64),
}

impl Expr {
    pub const E: Expr = Expr::Num(E);

    pub fn is_unit(&self) -> bool {
        self.is_num() || self.is_var()
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Self::Num(_))
    }

    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var)
    }

    pub fn precedence(&self) -> usize {
        match self {
            Expr::Bin(t, ..) => t.precedence(),
            Expr::Trig(..) => 3,
            Expr::Var | Expr::Num(_) => 100,
        }
    }

    pub fn evaluate(self, value: f64) -> f64 {
        match self {
            Expr::Bin(bin, a, b) => {
                let a = a.evaluate(value);
                let b = b.evaluate(value);

                match bin {
                    Bin::Add => a + b,
                    Bin::Sub => a - b,
                    Bin::Div => a / b,
                    Bin::Mul => a * b,
                    Bin::Exp => a.powf(b),
                    Bin::Log => a.log(b),
                }
            }
            Expr::Trig(trig, v) => {
                let (sin, cos) = v.evaluate(value).sin_cos();

                match trig {
                    Trig::Sin => sin,
                    Trig::Cos => cos,
                    Trig::Tan => sin / cos,
                    Trig::Csc => sin.recip(),
                    Trig::Sec => sin.recip(),
                    Trig::Cot => cos / sin,
                }
            }
            Expr::Var => value,
            Expr::Num(n) => n,
        }
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Num(value)
    }
}

impl From<usize> for Expr {
    fn from(value: usize) -> Self {
        Expr::Num(value as f64)
    }
}
