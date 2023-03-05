use crate::GuiHookVec::SortingList;

pub async fn cocktailShaker(list:&mut impl SortingList){
    let mut lowerBound = 0;
    let mut upperBound = list.len()-1;
    let mut swapped = true;
    while swapped{
        swapped = false;
        for i in lowerBound..upperBound {
            if list.lessThan(i+1, i) {
                if list.swap(i, i+1).await {return};
                swapped = true;
            }
        }
        if !swapped{
            break;
        }
        swapped = false;
        upperBound = upperBound-1;
        for i in ((lowerBound)..(upperBound)).rev() {
            if list.lessThan(i+1, i) {
                if list.swap(i+1, i).await {return};
                swapped = true;
            }
        }

        lowerBound = lowerBound+1;
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
  fn coctailshakersort_correct() {
    let mut list:NonGuiVec = aw!(SortingList::new(1000,0.0));
    aw!(cocktailShaker(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}