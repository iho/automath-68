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
            Forall(ref bound, ref left, ref right) => { write!(f, "({}: {}) -> {}", bound, left, right) }
            Universe(i) => write!(f, "Universe {}", i),
        }
    }
}

impl Henk {

    pub fn type_check(self) -> Result<Henk, String> {
        debug!("Start type checking.");
        self.type_check_with_context(HashMap::new())
    }

    pub fn type_check_with_context(self, context: HashMap<String, Henk>) -> Result<Henk, String> {
        println!("Type checking: {}", self);
        println!("Context: {:?}", context);
        match self {
            Henk::Universe(n) => Ok(Henk::Universe(n + 1)),
            Henk::Variable(v) => match context.get(&v) {
                Some(ty) => Ok(ty.clone()),
                None => Err(format!("Cannot find variable {}.", &v)),
            },
            Henk::Application(left, right) => {
                let left_ty = left.type_check_with_context(context.clone())?;
                match left_ty.whnf() {
                    Henk::Forall(bound, ty_in, ty_out) => {
                        let right_ty = right.clone().type_check_with_context(context.clone())?;
                        if right_ty.beta_eq(&ty_in) { Ok(ty_out.substitute(&bound, &right)) } else
                        { Err(format!("Expected something of type {}, found that of type {}.", ty_in, right_ty), ) }
                    }  left_ty => { Err(format!("Expected lambda, found value of type {}.", left_ty)) }
                }
            }
            Henk::Lambda(bound, ty, inner) => {
                ty.clone().type_check_with_context(context.clone())?;
                let mut new_context = context;
                new_context.insert(bound.clone(), *ty.clone());
                let inner_type = inner.type_check_with_context(new_context)?;
                Ok(Henk::Forall(bound, ty, Box::new(inner_type)))
            }
            Henk::Forall(bound, left, right) => {
                let left_sort = left.clone().type_check_with_context(context.clone()).map(Henk::whnf)?;
                let mut new_context = context;
                new_context.insert(bound.clone(), *left.clone());
                let right_kind = right.clone().type_check_with_context(new_context).map(Henk::whnf)?;
                Ok(Henk::Forall(bound.clone(), left.clone(), right))
            }
        }
    }

    pub fn free_vars(&self) -> HashSet<&String> {
        fn  closure_vars<'a>(bound: &'a String, ty: &'a Box<Henk>, inner: &'a Box<Henk>) -> HashSet<&'a String> {
            let mut tmp = inner.free_vars();
            tmp.remove(bound);
            tmp.union(&ty.free_vars()).cloned().collect()
        }
        let mut set = HashSet::new();
        match *self {
            Henk::Universe(v) => {}
            Henk::Variable(ref v) => { set = HashSet::new(); set.insert(v); }
            Henk::Application(ref left, ref right) => { set = left.free_vars().union(&right.free_vars()).cloned().collect() }
            Henk::Lambda(ref bound, ref ty, ref inner) => { closure_vars(bound, ty, inner); }
            Henk::Forall(ref bound, ref left, ref right) => { closure_vars(bound, left, right); }
        }
        set
    }

    pub fn substitute(self, from: &String, to: &Henk) -> Henk {
        match self {
            Henk::Universe(v) => Henk::Universe(v),
            Henk::Variable(v) => { if v == *from { to.clone() } else { Henk::Variable(v) } }
            Henk::Application(left, right) => { Henk::Application(Box::new(left.substitute(from, to)), Box::new(right.substitute(from, to))) }
            Henk::Lambda(ref bound, ref ty, ref inner) => {
                if bound == from { Henk::Lambda(bound.clone(),Box::new(ty.clone().substitute(from, to)),inner.clone(),) }
                else {
                    if !to.free_vars().contains(bound) { Henk::Lambda(bound.clone(),Box::new(ty.clone().substitute(from, to)),Box::new(inner.clone().substitute(from, to)),) }
                    else {
                        let mut unused: String = bound.clone();
                        unused.push_str("'");
                        loop {
                            let used: HashSet<&String> = inner.free_vars().union(&to.free_vars()).cloned().collect();
                            if used.contains(&unused) { unused.push_str("'") }
                            else {
                                let renamed = Henk::Lambda(unused.clone(), Box::new(ty.clone().substitute(bound, &Henk::Variable(unused.clone()), )),
                                    Box::new(inner.clone().substitute(bound, &&Henk::Variable(unused)), ), );
                                return renamed.substitute(from, to);
                            }
                        }
                    }
                }
            }
            Henk::Forall(ref bound, ref left, ref right) => {
                if bound == from { Henk::Forall(bound.clone(),Box::new(left.clone().substitute(from, to)),right.clone(),) }
                else {
                    if !to.free_vars().contains(bound) { Henk::Forall(bound.clone(), Box::new(left.clone().substitute(from, to)), Box::new(right.clone().substitute(from, to)),) }
                    else {
                        let mut unused: String = bound.clone();
                        unused.push_str("'");
                        loop {
                            let used: HashSet<&String> = right.free_vars().union(&to.free_vars()).cloned().collect();
                            if used.contains(&unused) { unused.push_str("'") } else {
                                let renamed = Henk::Forall(unused.clone(), Box::new(left.clone().substitute(bound,&&&Henk::Variable(unused.clone()),)), Box::new(right.clone().substitute(bound, &&Henk::Variable(unused)),), );
                                return renamed.substitute(from, to);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn whnf(self) -> Henk {
        fn spine(leftmost: Henk, stack: &[Henk]) -> Henk {
            match (leftmost, stack) {
                (Henk::Application(left, right), _) => {
                    let mut new_stack: Vec<Henk> = stack.into();
                    new_stack.push(*right);
                    spine(*left, &new_stack)
                }
                (Henk::Lambda(ref from, _, ref inner), ref stack) if !stack.is_empty() => {
                    let mut new_stack: Vec<Henk> = (*stack).into();
                    let right = new_stack.pop().unwrap();
                    spine(inner.clone().substitute(&from, &right), &new_stack)
                }
                (leftmost, _) => stack.iter().fold(leftmost, |l, r| Henk::Application(Box::new(l), Box::new(r.clone()))),
            }
        }
        spine(self, &[])
    }

    pub fn nf(self) -> Henk {
        fn spine(leftmost: Henk, stack: &[Henk]) -> Henk {
            match (leftmost, stack) {
                (Henk::Application(left, right), _) => {
                    let mut new_stack: Vec<Henk> = stack.into();
                    new_stack.push(*right);
                    spine(*left, &new_stack)
                }
                (Henk::Lambda(ref from, ref ty, ref inner), ref stack) if stack.is_empty() => {
                    Henk::Lambda(from.clone(), Box::new(ty.clone().nf()), Box::new(inner.clone().nf()), )
                }
                (Henk::Lambda(ref from, _, ref inner), ref stack) => {
                    let mut new_stack: Vec<Henk> = (*stack).into();
                    let right = new_stack.pop().unwrap();
                    spine(inner.clone().substitute(&from, &right), &new_stack)
                }
                (Henk::Forall(ref bound, ref left, ref inner), ref stack) => stack.iter().fold(
                    Henk::Forall(bound.clone(), Box::new(left.clone().nf()), Box::new(inner.clone().nf()), ),
                    |l, r| Henk::Application(Box::new(l), Box::new(r.clone().nf()))
                ),
                (leftmost, _) => stack.iter().fold(leftmost, |l, r| Henk::Application(Box::new(l), Box::new(r.clone().nf())) ),
            }
        }
        spine(self, &[])
    }

    pub fn alpha_eq(&self, another: &Henk) -> bool {
        match (self, another) {
            (&Henk::Universe(v1), &Henk::Universe(v2)) => v1 == v2,
            (&Henk::Variable(ref v1), &Henk::Variable(ref v2)) => v1 == v2,
            (&Henk::Application(ref left1, ref right1),&Henk::Application(ref left2, ref right2),) => left1.alpha_eq(&left2) && right1.alpha_eq(&right2),
            (&Henk::Lambda(ref bound1, ref ty1, ref inner1),&Henk::Lambda(ref bound2, ref ty2, ref inner2),) => { ty1.alpha_eq(ty2) && inner1.alpha_eq(&inner2.clone().substitute(&bound2, &&&Henk::Variable(bound1.clone())),) }
            (&Henk::Forall(ref bound1, ref left1, ref right1),&Henk::Forall(ref bound2, ref left2, ref right2),) => { left1.alpha_eq(left2) && right1.alpha_eq(&right2.clone().substitute(&bound2, &Henk::Variable(bound1.clone())),) }
            _ => false,
        }
    }

    pub fn beta_eq(&self, another: &Henk) -> bool {
        self.clone().nf().alpha_eq(&another.clone().nf())
    }
}
