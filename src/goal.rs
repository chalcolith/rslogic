//! # Logical Goals
//!
//! Goals are used to specify logical statements.

use state::{Unif, Var, State, PossibleStates};
use std::marker::PhantomData;

/// Evaluate a `Goal` to produce zero or more `State`s, or
/// collections of variable bindings.
pub trait Goal<T> where T: PartialEq + Unif<T> {
    fn eval(&self, state: &State<T>) -> PossibleStates<T>;
}


/// Evaluating a `Fail` goal always results in zero states.
pub struct Fail<T> where T: PartialEq + Unif<T> {
    _m: PhantomData<T>,
}

impl<T> Goal<T> for Fail<T> where T: PartialEq + Unif<T> {
    fn eval(&self, _: &State<T>) -> PossibleStates<T> {
        Vec::with_capacity(0)
    }
}

/// Creates a `Fail` goal.
pub fn fail<T>() -> Fail<T> where T: PartialEq + Unif<T> {
    Fail { _m: PhantomData }
}


/// Evaluating a `UnifyVal` goal attempts to unify a variable and a value.
pub struct UnifyVal<T> where T: PartialEq + Unif<T> {
    var: Var,
    val: T,
}

impl<T> Goal<T> for UnifyVal<T> where T: Clone + Eq + Unif<T> {
    fn eval(&self, state: &State<T>) -> PossibleStates<T> {
        state.unify_val(&self.var, self.val.clone())
    }
}

/// Creates a `UnifyVal` goal that attempts to unify the variable and the value.
pub fn unify_val<T>(var: &Var, val: T) -> UnifyVal<T> where T: PartialEq + Unif<T> {
    UnifyVal { var: *var, val: val }
}


/// Evaluating a `UnifyVar` goal attempts to unify the variables.
pub struct UnifyVar<T> where T: PartialEq + Unif<T> {
    v1: Var,
    v2: Var,
    _m: PhantomData<T>,
}

impl<T> Goal<T> for UnifyVar<T> where T: PartialEq + Unif<T> {
    fn eval(&self, state: &State<T>) -> PossibleStates<T> {
        state.unify_var(&self.v1, &self.v2)
    }
}

/// Creates a `UnifyVar` goal that attempts to unify the variables.
pub fn unify_vars<T>(v1: &Var, v2: &Var) -> UnifyVar<T> where T: PartialEq + Unif<T> {
    UnifyVar { v1: *v1, v2: *v2, _m: PhantomData }
}


/// A `Conjunction` goal evaluates its sub-goal `a` using a given state,
/// then evaluates sub-goal `b` using the results.
pub struct Conjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    a: A,
    b: B,
    _m: PhantomData<T>,
}

impl<T, A, B> Goal<T> for Conjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    fn eval(&self, state: &State<T>) -> PossibleStates<T> {
        let ra = self.a.eval(state);
        let mut result : Vec<State<T>> = Vec::with_capacity(0);
        for s in ra {
            let mut rb = self.b.eval(&s);
            result.append(&mut rb);
        }
        result
    }
}

/// Creates a `Conjunction` goal which returns the conjunction (logical AND) of evaluating the two sub-goals.
pub fn conj<T, A, B>(a: A, b: B) -> Conjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    Conjunction { a: a, b: b, _m: PhantomData }
}


/// Evaluating a `Disjunction` goal returns all the possible states of evaluating `a` and `b`.
pub struct Disjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    a: A,
    b: B,
    _m: PhantomData<T>,
}

impl<T, A, B> Goal<T> for Disjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    fn eval(&self, state: &State<T>) -> PossibleStates<T> {
        let mut da = self.a.eval(state).into_iter();
        let mut db = self.b.eval(state).into_iter();
        let mut result: Vec<State<T>> = Vec::with_capacity(0);
        loop {
            let sa = da.next();
            let sb = db.next();

            let mut found = false;
            if let Some(state) = sa { result.push(state); found = true; }
            if let Some(state) = sb { result.push(state); found = true; }

            if !found { break; }
        }
        result
    }
}

/// Creates a `Disjunction` goal which returns the disjunction (logical OR) of evaluating the two sub-goals.
pub fn disj<T, A, B>(a: A, b: B) -> Disjunction<T, A, B> where T: PartialEq + Unif<T>, A: Goal<T>, B: Goal<T> {
    Disjunction { a: a, b: b, _m: PhantomData }
}


/// Evaluating a `Predicate` goal returns the given state only if the function returns `true`.
pub struct Predicate<'a, T, F> where T: PartialEq + Unif<T>, F: Fn(&State<T>) -> bool + 'a {
    f: &'a F,
    _m: PhantomData<T>,
}

impl<'a, T, F> Goal<T> for Predicate<'a, T, F> where T: PartialEq + Unif<T>, F: Fn(&State<T>) -> bool {
    fn eval(&self, state: &State<T>) -> PossibleStates<T> {
        let f = self.f;
        if f(state) {
            vec![state.clone()]
        } else {
            Vec::with_capacity(0)
        }
    }
}

/// Creates a `Predicate` goal that filters a set of possible states with the given function.
pub fn pred<'a, T, F>(f: &'a F) -> Predicate<'a, T, F> where T: PartialEq + Unif<T>, F: Fn(&State<T>) -> bool {
    Predicate { f: f, _m: PhantomData }
}


macro_rules! unif_prim {
    ( $t:ty ) => {
        impl Unif<$t> for $t {
            fn unify(&self, other: &$t, prev: &State<$t>) -> PossibleStates<$t> {
                if self.eq(other) { vec![prev.clone()] } else { PossibleStates::new() }
            }
        }
    }
}

unif_prim!(bool);
unif_prim!(char);
unif_prim!(f32);
unif_prim!(f64);
unif_prim!(i16);
unif_prim!(i32);
unif_prim!(i64);
unif_prim!(i8);
unif_prim!(isize);
unif_prim!(u16);
unif_prim!(u32);
unif_prim!(u64);
unif_prim!(u8);
unif_prim!(usize);
unif_prim!(String);


#[cfg(test)]
mod tests {
    use state::{State};
    use super::{Goal, fail, unify_val, unify_vars, conj, disj, pred};

    #[test]
    fn test_bind_val() {
        let s = State::<i32>::empty();
        let (v, s) = s.make_var();

        let n: i32 = 34;
        let g = unify_val(&v, n);

        let results = g.eval(&s);
        assert_eq!(results.len(), 1);

        let val = results[0].get(&v).unwrap();
        assert_eq!(val, &n);
    }

    #[test]
    fn test_bind_var() {
        let s = State::<i32>::empty();
        let (a, s) = s.make_var();
        let (b, s) = s.make_var();

        let n: i32 = 12;
        let g1 = unify_vars(&a, &b);
        let g2 = unify_val(&b, n);
        let g = conj(g1, g2);

        let results = g.eval(&s);
        assert_eq!(results.len(), 1);

        let val = results[0].get(&a).unwrap();
        assert_eq!(val, &n);
    }

    #[test]
    fn test_conj_fail() {
        let s = State::<i32>::empty();
        let (v, s) = s.make_var();
        let g1 = unify_val(&v, 56);
        let g2 = fail::<i32>();
        let g = conj(g1, g2);

        let results = g.eval(&s);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_disj_fail() {
        let s = State::<i32>::empty();
        let (v, s) = s.make_var();
        let g1 = fail::<i32>();
        let g2 = unify_val(&v, 43);
        let g = disj(g1, g2);

        let results = g.eval(&s);
        assert_eq!(results.len(), 1);

        let val = results[0].get(&v).unwrap();
        assert_eq!(val, &43);
    }

    #[test]
    fn test_disj() {
        let s = State::<i32>::empty();
        let (a, s) = s.make_var();

        let g1 = unify_val(&a, 123);
        let g2 = unify_val(&a, 456);
        let g = disj(g1, g2);

        let results = g.eval(&s);
        assert_eq!(results.len(), 2);

        let val = results[0].get(&a).unwrap();
        assert_eq!(val, &123);

        let val = results[1].get(&a).unwrap();
        assert_eq!(val, &456);
    }

    #[test]
    fn test_pred() {
        let s = State::<i32>::empty();
        let (a, s) = s.make_var();

        let d = disj(unify_val(&a, 123), unify_val(&a, 987));
        let f = |s: &State<i32>| match s.get(&a) { Some(n) => *n == 987, None => false };
        let p = pred(&f);
        let g = conj(d, p);

        let results = g.eval(&s);
        assert_eq!(results.len(), 1);

        let val = results[0].get(&a).unwrap();
        assert_eq!(val, &987);
    }
}
