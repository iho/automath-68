#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Universe(i64),
    Lam(String, i64, Box<Expr>),
    Pi(String, i64, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Var(String, i64),
}
pub fn skip_first_char(s: &str) -> String {
    s.chars().skip(1).collect()
}
