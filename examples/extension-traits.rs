use std::{collections::HashSet, hash::Hash};

trait IteratorExt: Iterator {
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        UniqueIterator {
            i: self,
            seen: HashSet::new(),
        }
    }
}

struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    i: I,
    seen: HashSet<I::Item>,
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.i.find(|item| self.seen.insert(item.clone()))
    }
}

impl<I: Iterator> IteratorExt for I {}

fn main() {
    let arr = [1, 2, 3, 3, 4, 4, 5];
    let unique_arr: Vec<_> = arr.iter().unique().collect();
    println!("origin arr {:?}, unique arr {:?}", arr, unique_arr);
}
