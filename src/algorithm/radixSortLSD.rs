

use crate::GuiHookVec::SortingList;



pub async fn radixSort(list:&mut impl SortingList) {

    let mut max = usize::MIN;
    for i in list.getListClone().into_iter().map(|x| format!("{:b}",x.position)){

        if max < i.len(){
            max = i.len();
        }
 
    }
    for i in 0..(max){
        if radix(list, i).await {return};
    }
}

pub async fn radix(list:&mut impl SortingList, radix:usize) -> bool{
    let mut bucket = vec![vec![];2];

    for (i, bar) in list.elements().enumerate(){

        let cur = if get_bit_at(bar.position, radix) {1} else {0};
  
        bucket[cur].push(i);
        
    }


    let mut sortedIndexes = Vec::new();
    for elements in bucket.into_iter(){
        for i in elements{
            sortedIndexes.push(i);
        }
    }

    let mut listClone = list.getListClone();
    let mut count = 0;
    for i in sortedIndexes.clone(){
        if list.set(count, listClone[i]).await {return true};
        count += 1;

    }

    false
}

fn get_bit_at(input: usize, n: usize) -> bool {

    if format!("{:b}", input).len() <= n{
        false
    }else{
        input & (1 << n) != 0
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
  fn radixsort_correct() {
    let mut list:NonGuiVec = SortingList::new(1000,0.0);
    aw!(radixSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }

  #[test]
  fn get_bit_at_test(){
    let num = 0b0001;
    let othernum = 2;
    assert_eq!(get_bit_at(num, 0), true);
    assert_eq!(get_bit_at(num, 1), false);
    assert_eq!(get_bit_at(othernum, 1), true);
    assert_eq!(get_bit_at(othernum, 2), false);
    assert_eq!(get_bit_at(othernum, 3), false);
  }
}

