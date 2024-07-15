use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) {
    let res = grammar::ExprParser::new().parse(s).unwrap();
    println!("Term: {:?}", res);
    println!("Type: {:?}", res.type_check());
    println!("");
}
fn main() {

/*
  λ (x : ∀(Nat : *) → ∀ (Succ : Nat → Nat) → ∀ (Zero : Nat) → Nat)
→ x (∀ (Nat : *) → ∀ (Succ : Nat → Nat) → ∀ (Zero : Nat) → Nat)
    ( λ (pred : ∀(Nat : *) → ∀ (Succ : Nat → Nat) → ∀ (Zero : Nat) → Nat)
    → λ (Nat : *)
    → λ (Succ : Nat → Nat)
    → λ (Zero : Nat) → Succ (pred Nat Succ Zero))
*/

   print("(x:*) x");
   print("(x:*11) x");
   print("(A:*)(H:A)(T:[L:*][C:[_:A][_:L]L][N:L]L)(L:*)(C:[_:A][_:L]L)(N:L)(C H (T L C N))");
   print("(A: *) (Head: A) (Tail: [List: *] [Cons: [_:A] [_: List] List] [Nil: List] List) (List: *) (Cons: [_:A] [_: List] List) (Nil: List) (Cons Head (Tail List Cons Nil))");
   print("[Nat : *] [Succ : [_:Nat] Nat] [Zero : Nat] Nat");
   print("(pred : [Nat : *] [Succ : [_ : Nat] Nat] [Zero : Nat] Nat) (Nat : *) (Succ : [_: Nat] Nat) (Zero : Nat) (Succ (pred Nat Succ Zero))");
   print("[Nat:*] [Succ: [_:Nat] Nat] [Zero: Nat] Nat");
   print("(x: [Nat:*] [Succ: [_:Nat] Nat] [Zero: Nat] Nat) (x ([Nat:*][Succ:[_:Nat]Nat][Zero:Nat]Nat) ((pred: [Nat:*] [Succ: [_:Nat] Nat] [Zero:Nat] Nat) (Nat2:*) (Succ2:[_:Nat2] Nat2) (Zero2: Nat2) (Succ2 (pred Nat2 Succ2 Zero2))))");

}
