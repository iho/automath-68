
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;


fn print(s: &str) {
    let k = grammar::ExprParser::new().parse(s).unwrap();
    println!("{:?}", k);
}


fn main() {
    // print("[x:alpha]false");
    print("not(for,not(p,1))");
    print("not(for([z,f]not(<z>p)))");
    // print("for([z,elt(s)]imp(<z>p,for([z',elt(s)]imp(<z'>p,eq(z,z')))))");
    print("proof(ex_object([N,object]ex_arrow([0,arrow]ex_arrow([s,arrow]
    and(and(maps(0,1,N),maps(s,N,N)),
      for_object([A,object]for_arrow([x,arrow]for_arrow([f,arrow]
        imp(and(maps(x,1,A),maps(f,A,A)),
          ex_unique_arrow([u,arrow]and(maps(u,N,A),
            ex_arrow([h,arrow]ex_arrow([k,arrow]
              and(comp(u,0,x),and(comp(u,s,k),comp(f,u,k))))))))))))))))");
    // print("* nat : TYPE := PN");
    // print("*   1 : nat  := EB");
    // print("*   x : nat  := ---");

    // print("x * successor : nat := PN ");
    // print("* 2         : nat := successor(1)");
    // print("* 3         : nat := successor(2)");
}