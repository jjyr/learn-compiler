use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A naive graph used for register allocation
#[derive(Default, Debug)]
pub struct Graph<T>(HashMap<T, HashSet<T>>);

impl<T: Eq + Hash + Default + Clone> Graph<T> {
    pub fn insert(&mut self, a: T, b: T) {
        self.0
            .entry(a.clone())
            .or_insert_with(Default::default)
            .insert(b.clone());
        self.0.entry(b).or_insert_with(Default::default).insert(a);
    }
}
