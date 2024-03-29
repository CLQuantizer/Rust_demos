use std::fmt::Display;
use std::ops::AddAssign;

pub struct MyFilter<I, P> {
    pub iterator: I,
    pub predicate: P,
}

pub struct MyMap<I, M> {
    pub iterator: I,
    pub mapper: M,
}

pub trait MyIterator{ 
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn my_filter<F>(self, predicate: F) -> MyFilter<Self, F>
    where
        F: Fn(&Self::Item) -> bool,
        Self: Sized,
    {
        MyFilter {
            iterator: self,
            predicate,
        }
    }
    
    fn my_map<M, R>(self, mapper: M) -> MyMap<Self, M>
    where
        M: Fn(&Self::Item) -> R,
        Self: Sized,
    {
        MyMap {
            iterator: self,
            mapper,
        }
    }
    
    fn my_sum(mut self) -> Self::Item
        where 
            Self: Sized, 
            Self::Item: AddAssign + Default + Copy
    {   
        let mut sum = Self::Item::default();
        while let Some(item) = self.next() {
            sum += item;
        }
        sum
    }

}

pub fn print_iterator<T: Display>(mut iterator: impl MyIterator<Item = T>) {
    // Remember that MyIterator is not integrated to Rust
    // you will not be able to use `for elt in iterator {`
    while let Some(x) = iterator.next() {
        print!("{} ", x);
    }
    println!();
}


