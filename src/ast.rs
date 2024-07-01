#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    AngleBracket(Box<String>, Box<Option<Expr>>),
    SquareBrackets(Box<Option<String>>, Box<Option<Expr>>, Box<Option<Expr>>),
    RoundBrackets(Box<String>, Box<Option<Expr>>, Vec<(Box<Option<Expr>>, Box<Option<String>>)>),
}
