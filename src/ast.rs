use std::collections::{HashMap, HashSet};
use std::fmt::{self, write, Debug, Display};
use std::hash::Hash;

use log::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Henk {
    Universe(i64),
    Variable(String),
    Application(Box<Henk>, Box<Henk>),
    Lambda(String, Box<Henk>, Box<Henk>),
    Forall(String, Box<Henk>, Box<Henk>),
}
impl Display for Henk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Henk::*;
        match *self {
            Variable(ref str) => write!(f, "{}", str),
            Application(ref left, ref right) => write!(f, "({} {})", left, right),
            Lambda(ref bound, ref ty, ref inner) => write!(f, "(\\{}: {}. {})", bound, ty, inner),
            Forall(ref bound, ref left, ref right) => {
                write!(f, "({}: {}) -> {}", bound, left, right)
            }
            Universe(i) => write!(f, "Universe {}", i),
        }
    }
}

#[macro_export]
macro_rules! var {
    ($str: expr) => {
        Henk::Variable($str.into())
    };
}

#[macro_export]
macro_rules! app {
    ($left: expr, $right: expr) => {
        Henk::Application(Box::new($left), Box::new($right))
    };
}

#[macro_export]
macro_rules! lam {
    ($bound: expr, $ty: expr, $inner: expr) => {
        Henk::Lambda($bound.into(), Box::new($ty))
    };
}

#[macro_export]
macro_rules! pi {
    ($bound: expr, $left: expr, $right: expr) => {
        Henk::Forall($bound.into(), Box::new($left), Box::new($right))
    };
}

// A degenerating case of the `Pi` constructor.
#[macro_export]
macro_rules! arrow {
    ($left: expr, $right: expr) => {
        pi!("x", $left, $right)
    };
}

#[macro_export]
macro_rules! sort {
    ($sort: expr) => {
        Term::Sort($sort)
    };
}

impl Henk {
    // The starting point of type checking.
    pub fn type_check(self) -> Result<Henk, String> {
        debug!("Start type checking.");
        self.type_check_with_context(HashMap::new())
    }

    // And the real implementation of the type checking.
    // We need to store typing information in a map called `context`.
    pub fn type_check_with_context(self, context: HashMap<String, Henk>) -> Result<Henk, String> {
        println!("Type checking: {}", self);
        println!("Context: {:?}", context);
        match self {
            Henk::Universe(n) => Ok(Henk::Universe(n + 1)),
            // Simply lookup the context if I hit a variable.
            Henk::Variable(v) => match context.get(&v) {
                Some(ty) => Ok(ty.clone()),
                None => Err(format!("Cannot find variable {}.", &v)),
            },
            // If I hit an application ...
            Henk::Application(left, right) => {
                // First see if the left hand side type checks.
                let left_ty = left.type_check_with_context(context.clone())?;
                // If `left_ty` isn't a function in its `whnf` form, output an error.-------------+
                match left_ty.whnf() {
                    //   |
                    Henk::Forall(bound, ty_in, ty_out) => {
                        //   |
                        // Let's then type check the right hand side.                             |
                        let right_ty = right.clone().type_check_with_context(context.clone())?; //|
                                                                                                // If the type of the right hand side matches the type of the argument of |
                                                                                                // the `Pi` type, substitute the return type with the right hand side.    |
                                                                                                // The return type can have free occurences of the bound variable because |
                                                                                                // now we are working with dependent types.                               |
                        if right_ty.beta_eq(&ty_in) {
                            //   |
                            Ok(ty_out.substitute(&bound, &right)) //   |
                        } else {
                            //   |
                            // If the types doesn't match, return an error.                       |
                            Err(
                                //   |
                                format!(
                                    //   |
                                    "Expected something of type {}, found that of type {}.", //   |
                                    ty_in,
                                    right_ty //   |
                                ), //   |
                            ) //   |
                        } //   |
                    } //   |
                    left_ty =>
                    //   |
                    {
                        Err(format!("Expected lambda, found value of type {}.", left_ty))
                    } // <----+
                }
            }
            // If I hit a lambda ...
            Henk::Lambda(bound, ty, inner) => {
                // Check if the type of the argument is well-formed, if it is, proceed ...
                ty.clone().type_check_with_context(context.clone())?;
                let mut new_context = context;
                // Insert the bound variable into the new context.
                new_context.insert(bound.clone(), *ty.clone());
                // And type check the right hand side of the lambda with the new context.
                let inner_type = inner.type_check_with_context(new_context)?;
                Ok(Henk::Forall(bound, ty, Box::new(inner_type)))
            }
            // If I hit a `Pi` ...
            Henk::Forall(bound, left, right) => {
                // First, type check the type of the bound variable.
                // It must be a `Sort`, otherwise output an error.
                let left_sort = left
                    .clone()
                    .type_check_with_context(context.clone())
                    .map(Henk::whnf)?;
                // Create a new context, the same as what we did in the case of `Lam`.
                let mut new_context = context;
                // Insert the bound variable.
                new_context.insert(bound.clone(), *left.clone());
                // type check the right hand side of the `Pi` with the new context.
                let right_kind = right
                    .clone()
                    .type_check_with_context(new_context)
                    .map(Henk::whnf)?;
                // Again, check if the type of the return type is a `Sort`.
                Ok(Henk::Forall(bound.clone(), left.clone(), right))
            }
        }
    }

    // This function returns the set of free variables in a Henk and is used during substitution.
    pub fn free_vars(&self) -> HashSet<&String> {
        let mut set = HashSet::new();
        match *self {
            Henk::Universe(v) => {}
            // If what we get is a variable ...
            Henk::Variable(ref v) => {
                set = HashSet::new();
                // Then the only free variable is itself.
                set.insert(v);
            }
            // If it's an application, just merge the free variables in both sides of the Henk.
            Henk::Application(ref left, ref right) => {
                set = left
                    .free_vars()
                    .union(&right.free_vars())
                    .cloned()
                    .collect()
            }
            // If it's a lambda ...
            Henk::Lambda(ref bound, ref ty, ref inner) => {
                // Get the free variables from the right hand side.
                let mut tmp = inner.free_vars();
                // And remove the bound variable (because it is bound).
                tmp.remove(&bound);
                // The type of the bound variable could also contain free variables.
                set = tmp.union(&ty.free_vars()).cloned().collect();
            }
            // If it's a `Pi`, we do exactly the same as we did in a lambda!
            Henk::Forall(ref bound, ref left, ref right) => {
                let mut tmp = right.free_vars();
                tmp.remove(&bound);
                set = tmp.union(&left.free_vars()).cloned().collect();
            }
        }
        // debug!("{} has free variables {:?}.", self, set);
        set
    }

    // This function substitutes all occurences of the variable `from` into the Henk `to`.
    pub fn substitute(self, from: &String, to: &Henk) -> Henk {
        match self {
            Henk::Universe(v) => Henk::Universe(v),
            // If the Henk going to be substituted is a variable, there are 2 possibilities:
            // 1. `v == from`, then we just return `to`.
            // 2. `v != from`, then we return the variable untouched.
            Henk::Variable(v) => {
                if v == *from {
                    to.clone()
                } else {
                    Henk::Variable(v)
                }
            }
            // If we hit an application, recursively substitute both sides.
            Henk::Application(left, right) => {
                app!(left.substitute(from, to), right.substitute(from, to))
            }
            // If we hit a lambda, hmmmmm, it's a hard case.
            Henk::Lambda(ref bound, ref ty, ref inner) => {
                // If the bound variable coincide with `from`, we just need to substitite in its
                // type.
                if bound == from {
                    Henk::Lambda(
                        bound.clone(),
                        Box::new(ty.clone().substitute(from, to)),
                        inner.clone(),
                    )
                }
                // If it doesn't ...
                else {
                    // If the bound variable doesn't occur in `to`, then we simply go on
                    // recursively.
                    if !to.free_vars().contains(bound) {
                        Henk::Lambda(
                            bound.clone(),
                            Box::new(ty.clone().substitute(from, to)),
                            Box::new(inner.clone().substitute(from, to)),
                        )
                    }
                    // And now the hardest part about substitution.
                    else {
                        // We create a mutable variable which should eventually be unused in both
                        // the right hand side of the lambda and `to`
                        let mut should_be_unused: String = bound.clone();
                        should_be_unused.push_str("'");
                        loop {
                            let used: HashSet<&String> =
                                inner.free_vars().union(&to.free_vars()).cloned().collect();
                            // If `should_be_unused` actually is used, append the name of the
                            // variable with an apostrophe.
                            // Notice we're in a loop, so apostrophes will be appended indefinitely
                            if used.contains(&should_be_unused) {
                                should_be_unused.push_str("'")
                            }
                            // If `should_be_unused` literally isn't used ...
                            else {
                                // We change the Strings of the lambda from the clashed ones to the
                                // unused ones.
                                let renamed = Henk::Lambda(
                                    should_be_unused.clone(),
                                    Box::new(ty.clone().substitute(
                                        bound,
                                        &Henk::Variable(should_be_unused.clone()),
                                    )),
                                    Box::new(
                                        inner
                                            .clone()
                                            .substitute(bound, &&Henk::Variable(should_be_unused)),
                                    ),
                                );
                                // And then we do the real substitution.
                                return renamed.substitute(from, to);
                            }
                        }
                    }
                }
            }
            // `Pi` types are dealt with very similar to lambdas are.
            // I copy-pasted the code for the sake of not overengineering.
            Henk::Forall(ref bound, ref left, ref right) => {
                if bound == from {
                    Henk::Forall(
                        bound.clone(),
                        Box::new(left.clone().substitute(from, to)),
                        right.clone(),
                    )
                } else {
                    if !to.free_vars().contains(bound) {
                        Henk::Forall(
                            bound.clone(),
                            Box::new(left.clone().substitute(from, to)),
                            Box::new(right.clone().substitute(from, to)),
                        )
                    } else {
                        let mut should_be_unused: String = bound.clone();
                        should_be_unused.push_str("'");
                        loop {
                            let used: HashSet<&String> =
                                right.free_vars().union(&to.free_vars()).cloned().collect();
                            if used.contains(&should_be_unused) {
                                should_be_unused.push_str("'")
                            } else {
                                let renamed = Henk::Forall(
                                    should_be_unused.clone(),
                                    Box::new(left.clone().substitute(
                                        bound,
                                        &&&Henk::Variable(should_be_unused.clone()),
                                    )),
                                    Box::new(
                                        right
                                            .clone()
                                            .substitute(bound, &&Henk::Variable(should_be_unused)),
                                    ),
                                );
                                return renamed.substitute(from, to);
                            }
                        }
                    }
                }
            }
        }
    }

    // The purpose of this function is to get the *Weak Head Normal Form* of a Henk.
    pub fn whnf(self) -> Henk {
        // Basically, the **spine** of the syntax tree will be evaluated in this function.
        fn spine(leftmost: Henk, stack: &[Henk]) -> Henk {
            match (leftmost, stack) {
                // If we hit an application ...
                (Henk::Application(left, right), _) => {
                    let mut new_stack: Vec<Henk> = stack.into();
                    // Push the right hand side onto the stack ...
                    new_stack.push(*right);
                    // And then recurse.
                    spine(*left, &new_stack)
                }
                // If we hit a lambda and the stack isn't empty ...
                (Henk::Lambda(ref from, _, ref inner), ref stack) if !stack.is_empty() => {
                    let mut new_stack: Vec<Henk> = (*stack).into();
                    // Unwrapping here after popping is safe because `stack` isn't empty.
                    let right = new_stack.pop().unwrap();
                    // We just need to substitite and go forward.
                    spine(inner.clone().substitute(&from, &right), &new_stack)
                }
                // We simply build the Henk again if we encounter anything else.
                (leftmost, _) => stack.iter().fold(leftmost, |l, r| app!(l, r.clone())),
            }
        }
        spine(self, &[])
    }

    // In comparison with `whnf`, we evaluate every reducible expressions in the Henk.
    // The definition of the function `nf` is very similar to that of `whnf`,
    // but merging them into 1 function also seems like overengineering right now.
    pub fn nf(self) -> Henk {
        fn spine(leftmost: Henk, stack: &[Henk]) -> Henk {
            match (leftmost, stack) {
                // The same as above.
                (Henk::Application(left, right), _) => {
                    let mut new_stack: Vec<Henk> = stack.into();
                    new_stack.push(*right);
                    spine(*left, &new_stack)
                }
                // If the stack is empty, just recurse everywhere.
                (Henk::Lambda(ref from, ref ty, ref inner), ref stack) if stack.is_empty() => {
                    Henk::Lambda(
                        from.clone(),
                        Box::new(ty.clone().nf()),
                        Box::new(inner.clone().nf()),
                    )
                }
                // If the stack isn't empty, we do the same as above.
                (Henk::Lambda(ref from, _, ref inner), ref stack) => {
                    let mut new_stack: Vec<Henk> = (*stack).into();
                    // Unwrapping here after popping is safe because `stack` isn't empty.
                    let right = new_stack.pop().unwrap();
                    spine(inner.clone().substitute(&from, &right), &new_stack)
                }
                // If we hit a `Pi`, we recurse everywhere and build the Henk again.
                (Henk::Forall(ref bound, ref left, ref inner), ref stack) => stack.iter().fold(
                    Henk::Forall(
                        bound.clone(),
                        Box::new(left.clone().nf()),
                        Box::new(inner.clone().nf()),
                    ),
                    |l, r| app!(l, r.clone().nf()),
                ),
                // We simply build the Henk again if we encounter anything else.
                // Oh, now we also recurse on the right hand side.
                (leftmost, _) => stack.iter().fold(leftmost, |l, r| app!(l, r.clone().nf())),
            }
        }
        spine(self, &[])
    }

    // Alpha equality between types.
    pub fn alpha_eq(&self, another: &Henk) -> bool {
        match (self, another) {
            (&Henk::Universe(v1), &Henk::Universe(v2)) => v1 == v2,
            (&Henk::Variable(ref v1), &Henk::Variable(ref v2)) => v1 == v2,
            (
                &Henk::Application(ref left1, ref right1),
                &Henk::Application(ref left2, ref right2),
            ) => left1.alpha_eq(&left2) && right1.alpha_eq(&right2),
            (
                &Henk::Lambda(ref bound1, ref ty1, ref inner1),
                &Henk::Lambda(ref bound2, ref ty2, ref inner2),
            ) => {
                ty1.alpha_eq(ty2)
                    && inner1.alpha_eq(
                        &inner2
                            .clone()
                            .substitute(&bound2, &&&Henk::Variable(bound1.clone())),
                    )
            }
            (
                &Henk::Forall(ref bound1, ref left1, ref right1),
                &Henk::Forall(ref bound2, ref left2, ref right2),
            ) => {
                left1.alpha_eq(left2)
                    && right1.alpha_eq(
                        &right2
                            .clone()
                            .substitute(&bound2, &Henk::Variable(bound1.clone())),
                    )
            }
            _ => false,
        }
    }

    // Beta equality between types.
    // To know 2 Henks are beta equal, all you have to do is to make sure their `nf`s are alpha
    // equal.
    pub fn beta_eq(&self, another: &Henk) -> bool {
        self.clone().nf().alpha_eq(&another.clone().nf())
    }
}
