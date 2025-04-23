use std::f64::consts::E;

#[derive(Clone, Copy, Debug)]
enum Trig {
    Sin,
    Cos,
    Tan,

    Csc,
    Sec,
    Cot,
}

#[derive(Clone, Debug)]
enum Expr {
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
    /// Add this expr to another.
    fn add(self, f: impl Into<Expr>) -> Self {
        Self::Add(Box::new(self), Box::new(f.into()))
    }

    /// Subtract this expr to another.
    fn sub(self, f: impl Into<Expr>) -> Self {
        Self::Sub(Box::new(self), Box::new(f.into()))
    }

    /// Divide this expr by another.
    fn div(self, f: impl Into<Expr>) -> Self {
        Self::Div(Box::new(self), Box::new(f.into()))
    }

    /// Multiply tis expr by another.
    fn mul(self, f: impl Into<Expr>) -> Self {
        Self::Mul(Box::new(self), Box::new(f.into()))
    }

    /// raise this expr to another power.
    /// If the expr is `x` and the other is `a`, the result will be `x^a`
    fn exp(self, f: impl Into<Expr>) -> Self {
        Self::Exp(Box::new(self), Box::new(f.into()))
    }

    /// log base expr of another value.
    /// If the expr is `x` and the other is `a`, the result will be `log_x (a)`
    fn log(self, f: impl Into<Expr>) -> Self {
        Self::Log(Box::new(self), Box::new(f.into()))
    }

    /// Apply a trig function to this value.
    fn trig(self, func: Trig) -> Self {
        Self::Trig(func, Box::new(self))
    }

    /// Will panics if trying to flip a non binary.
    fn flip(self) -> Self {
        match self {
            Expr::Add(a, b) => Expr::Add(b, a),
            Expr::Sub(a, b) => Expr::Sub(b, a),
            Expr::Div(a, b) => Expr::Div(b, a),
            Expr::Mul(a, b) => Expr::Mul(b, a),
            Expr::Exp(a, b) => Expr::Exp(b, a),
            Expr::Log(a, b) => Expr::Log(b, a),

            _ => panic!("Tried to flip a non binary value"),
        }
    }
}

impl Expr {
    fn derive(self) -> Self {
        match self {
            Self::Add(f, g) => f.derive().add(g.derive()),
            Self::Sub(f, g) => f.derive().sub(g.derive()),

            Self::Div(f, g) => f
                .clone()
                .derive()
                .mul(*g.clone())
                .add(f.mul(g.clone().derive()))
                .div(g.exp(2.0)),
            Self::Mul(f, g) => f.clone().derive().mul(*g.clone()).add(g.derive().mul(*f)),

            Self::Exp(f, g) => match (*f, *g) {
                // x^n -> nx^(n-1)
                (x @ Self::Var, a @ Self::Num(n)) => a.mul(x.exp(n - 1.0)),

                // f(x)^n
                (f, a @ Self::Num(n)) => a.mul(f.clone().exp(n - 1.0)).mul(f.derive()),

                // n^x
                (a @ Self::Num(_), x @ Self::Var) => Self::Num(1.0).div(x.mul(Self::Num(E).log(a))),

                // n^f(x)
                (a @ Self::Num(_), f) => a.clone().exp(f.clone()).mul(Self::Num(E).log(a)).mul(f.derive()),

                _ => panic!("Exponents not in the form of f(x)^n or n^f(x) where n is a number are too complicated can't compute"),
            },
            Self::Log(f, g) => match (*f, *g) {
                (a @ Self::Num(_), f) => f.clone().derive().div(f.mul(Expr::Num(E).log(a))),
                _ => panic!("Logs not in the form of log_n (f(x)) where n is a number are too complicated can't compute"),
            },

            Self::Trig(func, f) => match func {
                Trig::Sin => f.clone().trig(Trig::Cos),
                Trig::Tan => f.clone().trig(Trig::Sec).exp(2.0),
                Trig::Sec => f.clone().trig(Trig::Sec).mul(f.clone().trig(Trig::Tan)),

                Trig::Cos => f.clone().trig(Trig::Sin).mul(-1.0),
                Trig::Cot => f.clone().trig(Trig::Csc).exp(2.0).mul(-1.0),
                Trig::Csc => f.clone().trig(Trig::Csc).mul(f.clone().trig(Trig::Cot)).mul(-1.0),
            }.mul(f.derive()),

            Self::Var => Self::Num(1.0),
            Self::Num(_) => Self::Num(0.0),
        }
    }
}

fn main() {
    let e = Expr::Var.exp(4.0).mul(1.0);
    println!("{e:?}");
    println!("{:?}", e.derive());
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Num(value)
    }
}
