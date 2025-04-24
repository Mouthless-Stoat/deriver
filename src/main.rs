mod expr;
use expr::*;

fn main() {
    let e = Expr::Var
        .exp(5.0)
        .mul(1.4)
        .sub(Expr::Var.exp(2.0).mul(2.5))
        .sub(6.7);

    println!("{e}");
    println!("{}", e.derive());
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Num(value)
    }
}
