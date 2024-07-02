#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Henk {
    U(i64),
    Var(String),
    Pi(String, Box<Henk>, Box<Henk>),
    Lam(String, Box<Henk>, Box<Henk>),
    App(Box<Henk>, Box<Henk>),
}
