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
   print("(A:*)(H:A)(T:[L:*][C:[_:A][_:L]L][N:L]L)(L:*)(C:[_:A][_:L]L)(N:L)(C H (T L C N))");
   print("(A: *) (Head: A) (Tail: [List: *] [Cons: [_:A] [_: List] List] [Nil: List] List) (List: *) (Cons: [_:A] [_: List] List) (Nil: List) (Cons Head (Tail List Cons Nil))");
}
