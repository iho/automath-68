use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) { println!("{:?}", grammar::ExprParser::new().parse(s).unwrap()); }

fn main() {
   print(
"[A:*][B:A] (A B C D)");
   print(
"(A:*)(H:A)(T:[L:*][C:(A>L>L)][N:L]L)(L:*)(C:A>L>L)(N:L) (C H T (L C N))");
}

