use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) { println!("{:?}", grammar::ExprParser::new().parse(s).unwrap()); }

fn main() {
   print("[A:*][B:[_:A]*][x:A](B(x))");
   print("(A:*)(H:A)(T:[L:*][C:[_:A][_:L]L][N:L]L)(L:*)(C:[_:A][_:L]L)(N:L)(C H (T L C N))");
}
