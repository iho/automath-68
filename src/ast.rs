#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Star,
    Var(String),
    AngleBracket(String),
    SquareBrackets(String, Box<Expr>),
    RoundBrackets(String, Box<Expr>, Option<Box<Expr>>),
}
