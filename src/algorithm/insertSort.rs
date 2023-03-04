use crate::GuiHookVec::SortingList;

pub async fn insertSort(list:&mut impl SortingList){
    for index in 0..list.len(){
        let mut j = index;
        while j>0 && list.lessThan(j, j-1){
            if list.swap(j, j - 1).await {return};
            j -= 1;
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
  fn insertsort_correct() {
    let mut list:NonGuiVec = aw!(SortingList::new(1000,0.0));
    aw!(insertSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}