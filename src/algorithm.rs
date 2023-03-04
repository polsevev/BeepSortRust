mod radixSort;
mod insertSort;
mod bubbleSort;
mod binaryHeap;
mod coctailShaker;
mod quickSort;
mod bogoSort;
mod radixSortLSD;


use crate::GuiHookVec::GuiVec;
use crate::GuiHookVec::SortingList;



#[derive(Debug, Clone)]
pub struct Algorithm{
    algorithms:Vec<String>
}

impl Algorithm{
    pub fn new() -> Self{
        Algorithm { algorithms: vec![
            "insertSort".to_string(),
            "bubbleSort".to_string(),
            "bogoSort".to_string(),
            "cocktailShaker".to_string(),
            "binaryHeap".to_string(),
            "quickSort".to_string(),
            "radixSortMSD".to_string(),
            "radixSortLSD".to_string()
            ] }
    }
    pub async fn run(length:usize, delay:f32, functionName:String){
        let mut list:GuiVec = SortingList::new(length, delay);
        list.randomize();
        

        match functionName.as_str() {
            "insertSort" => insertSort::insertSort(&mut list).await,
            "bubbleSort" => bubbleSort::bubbleSort(&mut list).await,
            "bogoSort" => bogoSort::bogoSort(&mut list).await,
            "cocktailShaker" => coctailShaker::cocktailShaker(&mut list).await,
            "binaryHeap" => binaryHeap::binaryHeap(&mut list).await,
            "quickSort" => quickSort::quickSort(&mut list).await,
            "radixSortMSD" => radixSort::radixSort(&mut list).await,
            "radixSortLSD" => radixSortLSD::radixSort(&mut list).await,
            _ => panic!("No algorithm with that name implemented!")
        }



        if !list.done{
            list.show().await
        }
    }

    pub fn getAlgorithms(&self) -> &Vec<String>{
        &self.algorithms
    }


}

