mod util;
use util::MyIterator;
use util::MyMap;
use util::MyFilter;
use util::print_iterator;

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

impl<I, F> MyIterator for MyFilter<I, F>
where
    I: MyIterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.iterator.next() {
                if (self.predicate)(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }
}

impl<I, M, R> MyIterator for MyMap<I, M>
where
    I: MyIterator,
    M: Fn(&I::Item) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iterator.next(){
            Some((self.mapper)(&value))
        }else {
            None
        }

    }
}

fn main() {
    let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
   
    //clone, cuz we ain't cosuming this shit
    let filter_mapped= enumeration.clone()
        .my_map(|item| *item)
        .my_filter(|&item| item > 3 && item < 9)
        .my_filter(|&item| item % 2 == 0)
        .my_map(|item| format!{"val: {}", item});
    print_iterator(filter_mapped);

    let total = enumeration.clone().my_sum();    
    println!("Total: {}", total);
}

