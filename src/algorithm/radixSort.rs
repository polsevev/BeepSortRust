use crate::GuiHookVec::SortingList;



pub async fn radixSort(list:&mut impl SortingList) {
    
    let mut max = usize::MIN;
    for i in list.getListClone().into_iter().map(|x| x.position.to_string()){

        if max < i.len(){
            max = i.len();
        }
 
    }

    for i in 0..(max){
        if radix(list, i).await {return};
    }
}

async fn radix(list:&mut impl SortingList, radix:usize) -> bool{
    let mut bucket = vec![vec![];10];

    for (i, bar) in list.elements().enumerate(){
        
        let cur = bar.position.to_string().chars().rev().collect::<Vec<char>>();
        if cur.len() > radix{
            bucket[cur[radix].to_digit(10).unwrap() as usize].push(i);
        }else{
            bucket[0].push(i);
        }
        
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
    let mut list:NonGuiVec = SortingList::new(10000,0.0);
    aw!(radixSort(&mut list));
    assert_eq!( list.isSorted(), true);
  }
}