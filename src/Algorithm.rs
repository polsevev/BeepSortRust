
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::rc::Rc;
use std::thread::yield_now;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Algorithm{

}

impl Algorithm{

    pub async fn insertSort(length:i32, delay:f32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length, delay);
        list.randomize();
        for index in 0..list.len(){
            let mut j = index;
            while j>0 && list.lessThan(j, j-1){
                list.swap(j, j - 1).await;
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


    pub async fn bubbleSort(length:i32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        let n = list.len();
        for i in 0..n {
            for j in 0..(n - i - 1) {
                if list.lessThan(j + 1, j) {
                    list.swap(j, j + 1).await;
                }
            }
        }

    }

    pub async fn bogoSort(length:i32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        loop{
            list.draw().await;
            if list.isSorted() {
                break;
            }
            list.randomize();
        }
    }

    pub async fn cocktailShaker(length:i32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        let mut lowerBound = 0;
        let mut upperBound = list.len()-1;
        let mut swapped = true;
        while swapped{
            swapped = false;
            for i in lowerBound..upperBound {
                if list.lessThan(i+1, i) {
                    list.swap(i+1, i).await;
                    swapped = true;
                }
            }
            if !swapped{
                break;
            }
            swapped = false;
            upperBound = upperBound-1;
            for i in ((lowerBound)..(upperBound-1)).rev() {
                if list.lessThan(i+1, i) {
                    list.swap(i+1, i).await;
                    swapped = true;
                }
            }

            lowerBound = lowerBound+1;
        }



    }

    pub async fn binaryHeap(length:i32){
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        let mut indexMap:HashMap<i32, usize> = HashMap::new();
        let mut binHeap:BinaryHeap<i32> = BinaryHeap::new();
        list.randomize();
        let mut ind = 0;
        for bar in list.elements(){
            binHeap.push(bar.position);
            indexMap.insert(bar.position, ind);
            ind += 1;
        }
        for i in (0..list.len()).rev(){
            let bar = binHeap.pop().unwrap();
            let barIndex = *indexMap.get(&bar).unwrap();
            list.swap(i, barIndex).await;
            let temp = list.get(barIndex).position;
            indexMap.insert(temp, barIndex);

        }

    }

     */



}