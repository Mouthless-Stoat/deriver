mod expr;
use expr::*;

fn main() {
    let e = Expr::Var.exp(4.0).mul(5.0).add(Expr::Var.exp(1.0 / 2.0));
    println!("{e}");
    println!("{}", e.derive());
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Num(value)
    }
}
