
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;


use async_recursion::async_recursion;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use std::collections::BinaryHeap;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Algorithm{

}

impl Algorithm{

    pub async fn run(length:i32, delay:f32, functionName:String){
        let mut list = GuiVec::new(length, delay);
        list.randomize();

        match functionName.as_str() {
            "insertSort" => Algorithm::insertSort(&mut list).await,
            "bubbleSort" => Algorithm::bubbleSort(&mut list).await,
            "bogoSort" => Algorithm::bogoSort(&mut list).await,
            "cocktailShaker" => Algorithm::cocktailShaker(&mut list).await,
            "binaryHeap" => Algorithm::binaryHeap(&mut list).await,
            "quickSort" => Algorithm::quickSort(&mut list, 0, length as usize -1).await,
            _ => panic!("No algorithm with that name implemented!")
        }



        if !list.done{
            list.show().await
        }
    }

    pub async fn insertSort(list:&mut GuiVec){
        for index in 0..list.len(){
            let mut j = index;
            while j>0 && list.lessThan(j, j-1){
                if list.swap(j, j - 1).await {return};
                j -= 1;
            }
        }
    }
    /*
    pub fn stalinSort(length:i32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();

        let mut cur = 1;
        loop{
            if cur == list.len() {
                break;
            }
            if list.lessThan(cur, cur-1){
                list.delete(cur);
            }else{
                cur += 1;
            }
            yield list.clone();
        }

    }
    */

    pub async fn bubbleSort(list:&mut GuiVec){
        let n = list.len();
        for i in 0..n {
            for j in 0..(n - i - 1) {
                if list.lessThan(j + 1, j) {
                    if list.swap(j, j + 1).await {return};
                }
            }
        }

    }

    pub async fn bogoSort(list:&mut GuiVec){

        loop{
            if list.swap(0,0).await {return};
            if list.isSorted() {
                break;
            }
            list.randomize();
        }
    }

    pub async fn cocktailShaker(list:&mut GuiVec){
        let mut lowerBound = 0;
        let mut upperBound = list.len()-1;
        let mut swapped = true;
        while swapped{
            swapped = false;
            for i in lowerBound..upperBound {
                if list.lessThan(i+1, i) {
                    if list.swap(i+1, i).await {return};
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

    pub async fn binaryHeap(list:&mut GuiVec){

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

 

    pub async fn partition(list:&mut GuiVec, mut low:usize, mut high:usize, p:i32) -> i32{
        while low <= high{
            while list.list[low].position < p{
                low += 1;
            }
            while list.list[high].position > p{
                high -= 1;
            }
            if low <= high{
                if list.swap(low, high).await {return -1};
                low+=1;
                if high == 0{
                    return low as i32
                }
                high-=1;
            }
        }
        low as i32
    }

    #[async_recursion]
    pub async fn quickSort(list:&mut GuiVec, low:usize, high:usize) {
        if low>=high{
            return;
        }

        let p = list.list[0].position;
        let mut index = 0;
        let temp = Algorithm::partition(list, low, high, p).await;
        if temp < 0{
            return
        }else{
            index = temp as usize
        }

        Algorithm::quickSort(list, low, index-1).await;
        Algorithm::quickSort(list, index, high).await;
        

    }



}