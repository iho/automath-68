
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;


fn print(s: &str) {
    let k = grammar::ExprParser::new().parse(s).unwrap();
    println!("{:?}", k);
}


fn main() {
    print("*11");
    print("Πx11.*11");
    print("λy4.*11");
}