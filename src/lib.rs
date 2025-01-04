use std::collections::HashSet;
use std::hash::Hash;

/// A Set collection that maintains a deterministic insertion order.
#[derive(Debug, Clone)]
pub struct SeqSet<T>
where
    T: Eq + Hash + Clone,
{
    set: HashSet<T>, // For O(1) existence checks
    vec: Vec<T>,     // To maintain deterministic insertion order
}

impl<T> SeqSet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        SeqSet {
            set: HashSet::new(),
            vec: Vec::new(),
        }
    }

    /// Inserts a value into the set.
    ///
    /// Returns `true` if the value was not present in the set.
    /// Returns `false` if the value was already present.
    pub fn insert(&mut self, value: T) -> bool {
        if self.set.insert(value.clone()) {
            self.vec.push(value);
            true
        } else {
            false
        }
    }

    /// Checks if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    /// Removes a value from the set.
    ///
    /// Returns `true` if the value was present and removed.
    /// Returns `false` if the value was not present.
    pub fn remove(&mut self, value: &T) -> bool {
        if self.set.remove(value) {
            // Find the position of the value in the Vec and remove it.
            if let Some(pos) = self.vec.iter().position(|x| x == value) {
                self.vec.remove(pos);
            }
            true
        } else {
            false
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }


    /// Returns an iterator over the elements in insertion order.
    pub fn iter(&self) -> SeqSetIter<'_, T> {
        SeqSetIter {
            inner: self.vec.iter(),
        }
    }

    /// Returns a mutable iterator over the elements in insertion order.
    pub fn iter_mut(&mut self) -> SeqSetIterMut<'_, T> {
        SeqSetIterMut {
            inner: self.vec.iter_mut(),
        }
    }


    /// Clears all elements from the set.
    pub fn clear(&mut self) {
        self.set.clear();
        self.vec.clear();
    }
}


pub struct SeqSetIter<'a, T>
where
    T: Eq + Hash + Clone,
{
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for SeqSetIter<'a, T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a SeqSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;
    type IntoIter = SeqSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub struct SeqSetIterMut<'a, T>
where
    T: Eq + Hash + Clone,
{
    inner: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for SeqSetIterMut<'a, T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a mut SeqSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a mut T;
    type IntoIter = SeqSetIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct SeqSetIntoIter<T>
where
    T: Eq + Hash + Clone,
{
    inner: std::vec::IntoIter<T>,
}

impl<T> Iterator for SeqSetIntoIter<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for SeqSet<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;
    type IntoIter = SeqSetIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SeqSetIntoIter {
            inner: self.vec.into_iter(),
        }
    }
}
