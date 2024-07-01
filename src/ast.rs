#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Henk {
    Universe(i64),
    Variable(String),
    Application(Box<Henk>, Box<Henk>),
    Lambda(String, Box<Henk>, Box<Henk>),
    Forall(String, Box<Henk>, Box<Henk>)
}
