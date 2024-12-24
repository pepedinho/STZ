use super::structure::{Expr, Literal};

impl Expr {
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let Expr::Literal(Literal::Integer(value)) = self {
            Some(*value)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let Expr::Literal(Literal::Str(value)) = self {
            Some(value)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let Expr::Literal(Literal::Float(value)) = self {
            Some(*value)
        } else {
            None
        }
    }
}
