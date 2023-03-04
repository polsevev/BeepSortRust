use crate::GuiHookVec::SortingList;



pub async fn quickSort(list:&mut impl SortingList) {
    let mut stack:Vec<(usize,usize)> = Vec::new();

    let start = 0;
    let end = list.len()-1;

    stack.push((start,end));

    while stack.len() > 0{
        let (start,end) = stack.pop().unwrap();

        let p = list.get(end).position;
        let temp = partition(list, start, end, p).await;
        let pivot = if temp >= 0 {temp} else {return};
        
        if pivot != 0 && pivot as usize -1 > start{
            
            stack.push((start,pivot as usize-1));
        }

        if pivot as usize + 1 < end{
            stack.push((pivot as usize +1, end))
        }
    }

}



async fn partition(list:&mut impl SortingList, mut low:usize, mut high:usize, p:usize) -> i32{
    let mut pIndex = low;

    for i in low..high{
        if list.lessThanEqual(i, p){
            if list.swap(i, pIndex).await {return -1}
            pIndex += 1;
        }
    }
    if list.swap(pIndex, high).await {return -1};

    return pIndex as i32
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
  fn quicksort_correct() {
    let mut list:NonGuiVec = SortingList::new(1000,0.0);
    aw!(quickSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}