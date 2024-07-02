use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) {
    let res = grammar::ExprParser::new().parse(s).unwrap(); 
    println!("{:?}", res);
    println!("{:?}", res.type_check());
}
fn main() {
   print("[A:*][B:A] (A B)");
   print("(A:*)(H:A)(T:[L:*][C:A->(L->L)][N:L]L)(L:*)(C:A->(L->L))(N:L) C H (T L C N)");
}

