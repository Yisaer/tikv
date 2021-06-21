// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use std::cmp::Reverse;
use std::collections::{binary_heap, BinaryHeap};
use std::iter;

// todo: Make it TopN<T, const N: usize> when we update our Rust version
/// TopN is used to collect the largest `cap` items pushed in
pub struct TopN<T> {
    capacity: usize,
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> TopN<T> {
    /// Create a new `TopN` instance which can hold the largest `capacity` items
    pub fn new(capacity: usize) -> TopN<T> {
        TopN {
            capacity,
            heap: BinaryHeap::with_capacity(capacity + 1),
        }
    }

    /// Push `item` into this `TopN`
    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
        if self.heap.len() > self.capacity {
            self.heap.pop();
        }
    }

    /// Pop the smallest `item` from this `TopN`.
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(x)| x)
    }

    /// How many items are there in this `TopN` now.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// How many items can this `TopN` keep at most.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Return whether this `TopN` is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Peek the largest item in this `TopN`
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(x)| x)
    }
}

impl<T> IntoIterator for TopN<T> {
    type Item = T;

    // this is added for rust-clippy#1013
    #[allow(clippy::type_complexity)]
    type IntoIter = iter::Map<binary_heap::IntoIter<Reverse<T>>, fn(Reverse<T>) -> T>;

    // note: IntoIterator doesn't require the result in order, there is an `IntoIterSorted`, implement that if necessary
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter().map(|Reverse(x)| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_capacity() {
        let mut topn_zero_capacity = TopN::new(0);
        topn_zero_capacity.push(1);
        assert_eq!(topn_zero_capacity.len(), 0);
        assert_eq!(topn_zero_capacity.pop(), None);

        let mut topn_zero_capacity = TopN::new(0);
        topn_zero_capacity.push(1);
        let iter = topn_zero_capacity.into_iter();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_1_capacity() {
        let mut topn_one_capacity = TopN::new(1);
        topn_one_capacity.push(1);
        assert_eq!(topn_one_capacity.len(), 1);
        assert_eq!(topn_one_capacity.pop(), Some(1));
        assert_eq!(topn_one_capacity.pop(), None);
        topn_one_capacity.push(1);
        topn_one_capacity.push(2);
        assert_eq!(topn_one_capacity.len(), 1);
        assert_eq!(topn_one_capacity.pop(), Some(2));
        assert_eq!(topn_one_capacity.pop(), None);

        let mut topn_one_capacity = TopN::new(1);
        topn_one_capacity.push(1);
        topn_one_capacity.push(2);
        let mut iter = topn_one_capacity.into_iter();
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_trivial() {
        let mut fix_topn = TopN::new(5);
        fix_topn.push(1);
        fix_topn.push(2);
        fix_topn.push(3);
        fix_topn.push(6);
        fix_topn.push(5);
        fix_topn.push(4);
        assert_eq!(fix_topn.len(), 5);
        assert_eq!(fix_topn.pop(), Some(2));
        assert_eq!(fix_topn.pop(), Some(3));
        assert_eq!(fix_topn.pop(), Some(4));
        assert_eq!(fix_topn.pop(), Some(5));
        assert_eq!(fix_topn.pop(), Some(6));
        assert_eq!(fix_topn.pop(), None);

        let mut fix_topn = TopN::new(5);
        fix_topn.push(1);
        fix_topn.push(2);
        fix_topn.push(3);
        fix_topn.push(6);
        fix_topn.push(5);
        fix_topn.push(4);
        let mut v: Vec<_> = fix_topn.into_iter().collect();
        v.sort_unstable();
        assert_eq!(v.len(), 5);
        assert_eq!(v, vec![2, 3, 4, 5, 6])
    }
}
