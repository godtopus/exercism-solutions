use std::hash::Hash;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct CustomSet<T: PartialEq + Eq + Hash + Clone> {
    backing_set: HashSet<T>
}

impl<T: PartialEq + Eq + Hash + Clone> CustomSet<T> {
    pub fn new(data: Vec<T>) -> Self {
        CustomSet {
            backing_set: data.into_iter().collect()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.backing_set.is_empty()
    }

    pub fn contains(&self, entry: &T) -> bool {
        self.backing_set.contains(entry)
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.backing_set.is_subset(&other.backing_set)
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.backing_set.is_disjoint(&other.backing_set)
    }

    pub fn add(&mut self, entry: T) -> bool {
        self.backing_set.insert(entry)
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> Self {
        CustomSet::new(self.backing_set.intersection(&other.backing_set).cloned().collect())
    }

    pub fn difference(&self, other: &CustomSet<T>) -> Self {
        CustomSet::new(self.backing_set.difference(&other.backing_set).cloned().collect())
    }

    pub fn union(&self, other: &CustomSet<T>) -> Self {
        CustomSet::new(self.backing_set.union(&other.backing_set).cloned().collect())
    }
}