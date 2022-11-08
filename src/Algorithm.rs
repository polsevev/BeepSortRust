
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::rc::Rc;
use std::thread::yield_now;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;

#[derive(Debug, Clone)]
pub struct Algorithm{

}

pub enum AlgoEnum{
    InsertSort(Box<dyn Generator<Yield=GuiVec, Return=()>>),
}

impl Algorithm{

    pub fn insertSort(length:i32) -> impl Generator<Yield=GuiVec, Return=()>{
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        move ||{
            yield list.clone();
            for index in 0..list.len(){
                let mut j = index;
                while j>0 && list.lessThan(j, j-1){
                    yield list.swap(j, j-1);
                    j = j-1;
                }

            }


        }
    }
    pub fn stalinSort(length:i32) -> impl Generator<Yield=GuiVec, Return=()>{
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        move ||{
            yield list.clone();
            let mut cur = 1;
            loop{
                if cur == list.len() {
                    break;
                }
                yield list.clone();
                if list.lessThan(cur, cur-1){
                    list.delete(cur)
                }else{
                    cur += 1;
                }
                yield list.clone();
            }
        }
    }

    pub fn bubbleSort(length:i32) -> impl Generator<Yield=GuiVec, Return=()>{
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        move || {
            let n = list.len();
            for i in 0..n {
                for j in 0..(n - i - 1) {
                    if list.lessThan(j + 1, j) {
                        yield list.swap(j, j + 1);
                    }
                }
            }
        }
    }

    pub fn bogoSort(length:i32) -> impl Generator<Yield=GuiVec, Return=()>{
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        move || {
            loop{
                yield list.clone();
                if list.isSorted() {
                    break;
                }
                list.randomize();
            }

        }
    }

    pub fn cocktailShaker(length:i32) -> impl Generator<Yield=GuiVec, Return=()>{
        let mut list = GuiVec::new(screen_width(), screen_height(), length);
        list.randomize();
        move || {
            let mut lowerBound = 0;
            let mut upperBound = list.len()-1;
            let mut swapped = true;
            while swapped{
                swapped = false;
                for i in lowerBound..upperBound {
                    if list.lessThan(i+1, i) {
                        yield list.swap(i+1, i);
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
                        yield list.swap(i+1, i);
                        swapped = true;
                    }
                }

                lowerBound = lowerBound+1;
            }

        }

    }

}