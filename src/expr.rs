mod combinator;
mod derive;
mod display;

#[derive(Clone, Copy, Debug)]
pub enum Trig {
    Sin,
    Cos,
    Tan,

    Csc,
    Sec,
    Cot,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),

    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),

    /// 0 is base, 1 is exponent
    Exp(Box<Expr>, Box<Expr>),
    /// 0 is base, 1 is value
    Log(Box<Expr>, Box<Expr>),

    Trig(Trig, Box<Expr>),

    Var,
    Num(f64),
}

impl Expr {
    pub fn is_unit(&self) -> bool {
        self.is_num() || self.is_var()
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Self::Num(_))
    }

    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var)
    }
}
