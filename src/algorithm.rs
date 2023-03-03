mod radixSort;
mod insertSort;
mod bubbleSort;
mod binaryHeap;
mod coctailShaker;
mod quickSort;
mod bogoSort;


use crate::GuiHookVec::GuiVec;
use crate::GuiHookVec::SortingList;



#[derive(Debug, Clone)]
pub struct Algorithm{

}

impl Algorithm{

    pub async fn run(length:i32, delay:f32, functionName:String){
        let mut list:GuiVec = SortingList::new(length, delay);
        list.randomize();
        

        match functionName.as_str() {
            "insertSort" => insertSort::insertSort(&mut list).await,
            "bubbleSort" => bubbleSort::bubbleSort(&mut list).await,
            "bogoSort" => bogoSort::bogoSort(&mut list).await,
            "cocktailShaker" => coctailShaker::cocktailShaker(&mut list).await,
            "binaryHeap" => binaryHeap::binaryHeap(&mut list).await,
            "quickSort" => quickSort::quickSort(&mut list).await,
            "radixSort" => radixSort::radixSort(&mut list).await,
            _ => panic!("No algorithm with that name implemented!")
        }



        if !list.done{
            list.show().await
        }
    }


}

