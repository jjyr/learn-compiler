use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// A naive graph used for register allocation
#[derive(Default, Debug)]
pub struct Graph<T: Eq + Hash + Default + Debug>(HashMap<T, HashSet<T>>);

impl<T: Eq + Hash + Default + Clone + Debug> Graph<T> {
    /// insert a pair of adjacent vertex
    pub fn insert(&mut self, a: T, b: T) {
        self.0
            .entry(a.clone())
            .or_insert_with(Default::default)
            .insert(b.clone());
        self.0.entry(b).or_insert_with(Default::default).insert(a);
    }

    pub fn add_vertex(&mut self, a: T) {
        self.0.entry(a.clone()).or_insert_with(Default::default);
    }

    pub fn get_adjacents_set(&self, v: &T) -> Option<&HashSet<T>> {
        self.0.get(v)
    }

    /// remove a vertex
    pub fn remove(&mut self, v: &T) {
        if let Some(removed) = self.0.remove(v) {
            for i in removed {
                self.0.entry(i).or_default().remove(v);
            }
        }
    }

    pub fn iter_vertex(&self) -> impl Iterator<Item = &T> {
        self.0.keys()
    }
}
