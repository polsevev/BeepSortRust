use crate::GuiHookVec::SortingList;

pub async fn bubbleSort(list:&mut impl SortingList){
    let n = list.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if list.lessThan(j + 1, j) {
                if list.swap(j, j + 1).await {return};
            }
        }
    }

}
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use crate::GuiHookVec::NonGuiVec;

use super::*;
  

  macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

  #[test]
  fn bubblesort_correct() {
    let mut list:NonGuiVec = aw!(SortingList::new(1000,0.0));
    aw!(bubbleSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}