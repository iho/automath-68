use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) {
    let res = grammar::ExprParser::new().parse(s).unwrap(); 
    println!("{:?}", res);
    println!("{:?}", res.type_check());
}
fn main() {
   print("(x:*) x");
   print("(x:*11) x");
   print("(A: *) (Head: A) (Tail: [List: *] [Cons: A -> List -> List] [Nil: List] List)");
   print("(List: *) (Cons: A -> List -> List) (Nil: List) Cons Head (Tail List Cons Nil)");
   // print("[A:*][B:A] (A B)");
   // print("(A:*)(H:A)(T:[L:*][C:A->(L->L)][N:L]L)(L:*)(C:A->(L->L))(N:L) C H (T L C N)");
}

