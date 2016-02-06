//! # Rust Logic
//!
//! **[rslogic](https://github.com/kulibali/rslogic)** is a logic programming framework
//! for Rust inspired by [µKanren](https://github.com/jasonhemann/microKanren).
//!
//! A logical statement is built from **variable**s, **state**s, and **goal**s.
//! Create an initial state, then obtain some variables (and resulting state) from it.
//! Construct a goal consisting of variable bindings, logical operations (AND, OR), or
//! predicates.  Then evaluate the goal using the state resulting from making the variables.
//! Evaluating a goal returns all possible solutions to the statement, in the form of
//! a number of states containing variable bindings.
//!
//! ```
//! use rslogic::state;
//! use rslogic::state::{Unif, State, PossibleStates};
//! use rslogic::goal;
//! use rslogic::goal::{Goal, fail, unify_val, unify_vars, conj, disj, pred};
//!
//! let s = state::State::<i32>::empty();
//! let (v1, s) = s.make_var();
//! let (v2, s) = s.make_var();
//!
//! let n = 123;
//! let g = goal::conj(goal::unify_vars(&v1, &v2), goal::unify_val(&v2, n));
//!
//! let results = g.eval(&s);
//! assert_eq!(results.len(), 1);
//! let bound_value = results[0].get(&v1).unwrap();
//! assert_eq!(bound_value, &n);
//! ```
//!
//! This example creates two variables, `v1` and `v2`, and then assembles a logical expression
//! equivalent to `(v1 = v2) && (v2 = 123)`.  When evaluated, the resulting state binds `123` to
//! both `v1` and `v2`.
//!

mod btmap;
pub mod goal;
pub mod state;
