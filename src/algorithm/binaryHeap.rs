use std::collections::{HashMap, BinaryHeap};

use crate::GuiHookVec::SortingList;

pub async fn binaryHeap(list:&mut impl SortingList){

    let mut indexMap:HashMap<i32, usize> = HashMap::new();
    let mut binHeap:BinaryHeap<i32> = BinaryHeap::new();

    let mut ind = 0;
    for bar in list.elements(){
        binHeap.push(bar.position);
        indexMap.insert(bar.position, ind);
        ind += 1;
    }
    for i in (0..list.len()).rev(){
        let bar = binHeap.pop().unwrap();
        let barIndex = *indexMap.get(&bar).unwrap();
        if list.swap(i, barIndex).await {return};
        let temp = list.get(barIndex).position;
        indexMap.insert(temp, barIndex);

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
  fn binartHeap_correct() {
    let mut list:NonGuiVec = SortingList::new(1000,0.0);
    aw!(binaryHeap(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}
