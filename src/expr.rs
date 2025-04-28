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

#[derive(Clone, Copy, Debug)]
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

impl PartialEq for Bin {
    fn eq(&self, other: &Self) -> bool {
        self.precedence() == other.precedence()
    }
}

impl Eq for Bin {}

impl Ord for Bin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

impl PartialOrd for Bin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Bin(Bin, Box<Expr>, Box<Expr>),
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

    pub fn precedence(&self) -> usize {
        match self {
            Expr::Bin(t, ..) => t.precedence(),
            Expr::Trig(..) => 3,
            Expr::Var | Expr::Num(_) => 100,
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

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.precedence() == other.precedence()
    }
}

impl Eq for Expr {}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
