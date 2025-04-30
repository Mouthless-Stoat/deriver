mod basic {
    use crate::prelude::*;

    #[test]
    fn power_rule() {
        assert_eq!(Var.exp(10.0).derive(), Num(10.0).mul(Var.exp(9.0)))
    }

    #[test]
    fn power_rule_op_1() {
        assert_eq!(Var.exp(2.0).derive(), Num(2.0).mul(Var))
    }

    #[test]
    fn power_rule_op_2() {
        assert_eq!(Var.exp(1.0).derive(), Num(1.0));
    }

    #[test]
    fn constant_rule() {
        assert_eq!(
            Num(3.0).mul(Var.exp(2.0)).derive(),
            Num(3.0).mul(Num(2.0).mul(Var))
        )
    }

    #[test]
    fn sum_rule() {
        assert_eq!(
            Var.exp(2.0).add(Var.exp(3.0)).derive(),
            Num(2.0).mul(Var).add(Num(3.0).mul(Var.exp(2.0)))
        )
    }

    #[test]
    fn diff_rule() {
        assert_eq!(
            Var.exp(2.0).sub(Var.exp(3.0)).derive(),
            Num(2.0).mul(Var).sub(Num(3.0).mul(Var.exp(2.0)))
        )
    }

    #[test]
    fn product_rule() {
        let f = Var.exp(2.0);
        let g = Var.exp(3.0);
        let f_prime = Num(2.0).mul(Var);
        let g_prime = Num(3.0).mul(Var.exp(2.0));

        assert_eq!(
            f.clone().mul(g.clone()).derive(),
            f_prime.mul(g.clone()).add(g_prime.mul(f))
        )
    }

    #[test]
    fn quotient_rule() {
        let f = Var.exp(2.0);
        let g = Var.exp(3.0);
        let f_prime = Num(2.0).mul(Var);
        let g_prime = Num(3.0).mul(Var.exp(2.0));

        assert_eq!(
            f.clone().div(g.clone()).derive(),
            f_prime.mul(g.clone()).sub(g_prime.mul(f)).div(g.exp(2.0))
        )
    }

    #[test]
    fn ex_rule() {
        assert_eq!(Expr::E.exp(Var).derive(), Expr::E.exp(Var),)
    }

    #[test]
    fn ax_rule() {
        assert_eq!(
            Num(2.0).exp(Var).derive(),
            Num(2.0).exp(Var).mul(Num(2.0).ln()),
        )
    }

    #[test]
    fn ln_x_rule() {
        assert_eq!(Var.ln().derive(), Num(1.0).div(Var))
    }

    #[test]
    fn log_a_x_rule() {
        assert_eq!(Var.log(2.0).derive(), Num(1.0).div(Var.mul(Num(2.0).ln())))
    }

    #[test]
    fn log_x_a_rule() {
        assert_eq!(
            Num(2.0).log(Var).derive(),
            Num(2.0).ln().div(Var.mul(Var.ln().exp(2.0)))
        )
    }
}
