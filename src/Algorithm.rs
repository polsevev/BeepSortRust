
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;


use async_recursion::async_recursion;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::f32::consts::PI;


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
            "quickSort" => Algorithm::quickSort(&mut list).await,
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


    pub async fn quickSort(list:&mut GuiVec) {
        let mut stack:Vec<(usize,usize)> = Vec::new();
  
        let start = 0;
        let end = list.len()-1;

        stack.push((start,end));

        while stack.len() > 0{
            let (start,end) = stack.pop().unwrap();

            let p = list.list[end].position;
            let temp = Algorithm::partition(list, start, end, p).await;
            let pivot = if temp >= 0 {temp} else {return};
            
            if pivot != 0 && pivot as usize -1 > start{
                
                stack.push((start,pivot as usize-1));
            }

            if pivot as usize + 1 < end{
                stack.push((pivot as usize +1, end))
            }
        }

    }



}