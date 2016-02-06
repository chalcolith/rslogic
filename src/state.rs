//! # Logical States
//!
//! A logical state is a collection of variable bindings.
//!

use std::clone::Clone;

/// A collection of possible states.
pub type PossibleStates<T> = Vec<State<T>>;

/// Values used in a state must be unifiable.  Unifying two values produces
/// zero or more possible states, where variables that may be contained in the
/// values may be bound in various combinations.
pub trait Unif<T> where T : PartialEq + Unif<T> {
    fn unify(&self, other: &T, prev: &State<T>) -> PossibleStates<T>;
}

/// Represents a logical variable.  A variable must be created by calling
/// `State::make_var()` before a goal is evaluated (by passing the
/// resulting state to a goal).
#[derive(Clone, Copy)]
pub struct Var {
    index: usize,
}

use btmap::BtMap;

/// A logical state, containing a collection of variable bindings.
///
/// Variables are stored with one level of indirection, to indicate
/// variables that have been unified before being bound.
pub struct State<T> where T : PartialEq + Unif<T> {
    bindings: BtMap<usize, usize>, // var index -> slot
    slots: BtMap<usize, T>, // slot -> value
    next_index: usize,
}

impl<T> State<T> where T : PartialEq + Unif<T> {
    /// Creates an empty state.
    pub fn empty() -> State<T> {
        State {
            bindings: BtMap::empty(),
            slots: BtMap::empty(),
            next_index: 0
        }
    }

    /// Returns `true` if the variable is bound in the state.
    pub fn binds_var(&self, var: &Var) -> bool {
        match self.bindings.get(&var.index) {
            Some(ref slot) => self.slots.contains_key(slot),
            None => false
        }
    }

    /// Returns a reference to the value bound to the variable in the state,
    /// or None if the var4iable is not bound.
    pub fn get<'a>(&'a self, var: &Var) -> Option<&'a T> {
        match self.bindings.get(&var.index) {
            Some(ref slot) => self.slots.get(slot),
            None => None,
        }
    }

    /// Attempts to unify a variable with a value.  If the variable is not bound,
    /// returns a new state containing a binding to the value.  If the variable is
    /// already bound, returns the unification of the two values.
    pub fn unify_val(&self, var: &Var, val: T) -> PossibleStates<T> {
        match self.bindings.get(&var.index) {
            Some(slot) => {
                // if the variable has a slot (could be bound or unified with another variable)
                // see if it has a value.  if so, unify with the value, otherwise bind it to the value
                match self.slots.get(slot) {
                    Some(existing) => {
                        existing.unify(&val, self)
                    },
                    None => {
                        vec![State {
                            bindings: self.bindings.clone(),
                            slots: self.slots.insert(*slot, val).unwrap(),
                            .. *self
                        }]
                    }
                }
            },
            None => {
                // if this variable is not bound, make a new slot and binding for it
                let index = &var.index;
                vec![State {
                    bindings: self.bindings.insert(*index, *index).unwrap(),
                    slots: self.slots.insert(*index, val).unwrap(),
                    .. *self
                }]
            }
        }
    }

    /// Attempts to unify two variables.
    pub fn unify_var(&self, v1: &Var, v2: &Var) -> PossibleStates<T> {
        let b1 = self.bindings.get(&v1.index);
        let b2 = self.bindings.get(&v2.index);

        match b1 {
            Some(s1) => { // v1 has a slot
                match b2 {
                    Some(s2) => { // both variables have slots
                        let value1 = self.slots.get(s1);
                        let value2 = self.slots.get(s2);

                        match value1 {
                            Some(vv1) => {
                                match value2 {
                                    Some(ref vv2) => vv1.unify(vv2, self), // both v1 and v2 are bound, unify values
                                    None => PossibleStates::new()    // v2 is not bound, this is an error
                                }
                            },
                            None => {
                                match value2 {
                                    Some(_vv2) => PossibleStates::new(), // v1 is not bound, this is an error
                                    None => if s1.eq(s2) { vec![self.clone()] }
                                            else { PossibleStates::new() } // neither slot is bound; slots must then be equal
                                }
                            }
                        }
                    },
                    None => { // v1 has a slot, v2 does not
                        vec![State {
                            bindings: self.bindings.insert(v2.index, *s1).unwrap(),
                            slots: self.slots.clone(),
                            .. *self
                        }]
                    }
                }
            },
            None => { // v1 does not have a slot
                match b2 {
                    Some(s2) => { // v1 does not have a slot, v2 does
                        vec![State {
                            bindings: self.bindings.insert(v1.index, *s2).unwrap(),
                            slots: self.slots.clone(),
                            .. *self
                        }]
                    },
                    None => { // neither variable has a slot
                        let slot = &v1.index;
                        vec![State {
                            bindings: self.bindings
                                        .insert(v1.index, *slot).unwrap()
                                        .insert(v2.index, *slot).unwrap(),
                            slots: self.slots.clone(),
                            .. *self
                        }]
                    }
                }
            }
        }
    }

    /// Creates a new variable and a new state with which it is usable.
    pub fn make_var(&self) -> (Var, State<T>) {
        let var = Var { index: self.next_index };
        let state = State {
            bindings: self.bindings.clone(),
            slots: self.slots.clone(),
            next_index: self.next_index + 1
        };
        (var, state)
    }
}

impl<T> Clone for State<T> where T : PartialEq + Unif<T> {
    fn clone(&self) -> State<T> {
        State {
            bindings: self.bindings.clone(),
            slots: self.slots.clone(),
            next_index: self.next_index
        }
    }

    fn clone_from(&mut self, source: &State<T>) {
        self.bindings = source.bindings.clone();
        self.slots = source.slots.clone();
        self.next_index = source.next_index;
    }
}
