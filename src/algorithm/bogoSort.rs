use crate::GuiHookVec::SortingList;

pub async fn bogoSort(list:&mut impl SortingList){

    loop{
        if list.swap(0,0).await {return};
        if list.isSorted() {
            break;
        }
        list.randomize();
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
  fn bogoSort_correct_this_is_a_meme() {
    let mut list:NonGuiVec = aw!(SortingList::new(5,0.0));
    aw!(bogoSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}