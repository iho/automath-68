
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;


fn print(s: &str) {
    let k = grammar::ExprParser::new().parse(s).unwrap();
    println!("{:?}", k);
}


fn main() {
    print("[x:alpha]false");
    print("not(for([z,elt(s)]not(<z>p)))");
    print("for([z,elt(s)]imp(<z>p,for([z',elt(s)]imp(<z'>p,eq(z,z')))))");
    // print("* nat : TYPE := PN");
    // print("*   1 : nat  := EB");
    // print("*   x : nat  := ---");

    // print("x * successor : nat := PN ");
    // print("* 2         : nat := successor(1)");
    // print("* 3         : nat := successor(2)");
}