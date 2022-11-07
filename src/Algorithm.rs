
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;
use std::ops::{Generator, GeneratorState};
use std::rc::Rc;
use std::thread::yield_now;
#[derive(Debug, Clone)]
pub struct Algorithm{
    name:String,
}

impl Algorithm {

    pub fn new() -> Algorithm{
        Algorithm{name:"Test".to_owned()}
    }

    pub fn insertSort<'a>(list: &'a mut GuiVec) -> impl Generator<Yield=GuiVec, Return=()> +'a{
        move ||{
            yield list.clone();
            for index in 0..list.len(){
                let mut j = index;
                while j>0 && list.lessThan(j, j-1){
                    //yield list.clone();
                    yield list.swap(j, j-1);
                    //yield list.clone();
                    j = j-1;
                }
            }
        }
    }
    pub fn stalinSort<'a>(list: &'a mut GuiVec) -> impl Generator<Yield=GuiVec, Return=()> +'a{
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

}