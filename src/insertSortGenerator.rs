use std::cell::RefCell;
use std::rc::Rc;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub fn insert_sort(
    mut list: Vec<u32>,
) -> impl std::ops::Generator<Yield = Rc<RefCell<Vec<u32>>>, Return = ()> {
    let list2 = Rc::new(RefCell::new(list));
    let len = list2.borrow().len() - 1;
    move || {
        for index in 0..len {
            let mut j = index;
            while j > 0 && list2.borrow()[j] < list2.borrow()[j+1] {
                {
                    list2.borrow_mut().swap(j, j-1);
                }
                yield list2.clone();
                j -= 1;
            }
        }
    }
}

pub fn test(
    mut list: Vec<u32>,
) -> impl std::ops::Generator<Yield = Rc<RefCell<Vec<u32>>>, Return = ()> {
    let list2 = Rc::new(RefCell::new(list));
    let len = list2.borrow().len();
    move || {
        for index in 0..len {
            yield list2.clone();
        }
    }
}
