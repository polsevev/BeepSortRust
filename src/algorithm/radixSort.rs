use async_recursion::async_recursion;

use crate::GuiHookVec::SortingList;



pub async fn radixSort(list:&mut impl SortingList) {
    
    let mut max = usize::MIN;
    for i in list.getListClone().into_iter().map(|x| format!("{:b}",x.position)){

        if max < i.len(){
            max = i.len();
        }
 
    }
    

    let mut stack:Vec<(usize, usize)> = Vec::new();
    let mut stack2:Vec<(usize,usize)> = Vec::new();
    stack.push((0, list.len()));
    for i in (0..max).rev(){



        while stack.len() > 0{
            let cur = stack.pop().unwrap();
            print!("{:?}", cur);
            match radix(list, i, cur.0, cur.1).await{
                Some((initial, switch, max)) => {
                    if initial < switch{
                        stack2.push((initial, switch));
                    }
                    if switch < max{
  
                        stack2.push((switch, max));
                    }
 
                },
                None => return
            }
        }
        stack = stack2.clone();  
    }
}

async fn radix(list:&mut impl SortingList, radix:usize, mut minBoundry:usize, mut maxBoundry:usize) -> Option<(usize,usize,usize)>{
    let initialMin = minBoundry.clone();
    let initialMax = maxBoundry.clone();

    // let mut bitVec:Vec<usize> = Vec::new();
    // for i in 0..list.len(){
    //     let currentBit = get_bit_at(list.get(i).position, radix);
    //     bitVec.push(if currentBit {1} else {0});
    // }
    // println!("{:?}", bitVec);

    loop{
        if maxBoundry == minBoundry{
            break
        }
        let currentBit = get_bit_at(list.get(minBoundry).position, radix);

        if currentBit{
            if list.swap(minBoundry, maxBoundry-1).await {return None};
            maxBoundry -= 1;
        }else{
            if list.swap(minBoundry, minBoundry).await {return None};

            minBoundry += 1;
        }
    }

    // let mut bitVec:Vec<usize> = Vec::new();
    // for i in 0..list.len(){
    //     let currentBit = get_bit_at(list.get(i).position, radix);
    //     bitVec.push(if currentBit {1} else {0});
    // }
 
    
    Some((initialMin, minBoundry, initialMax))
}
fn get_bit_at(input: usize, n: usize) -> bool {
    println!("{} >= {}", format!("{:b}", input).len(), n);
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

