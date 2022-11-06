
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;
use std::ops::{Generator, GeneratorState};
use std::rc::Rc;
use std::thread::yield_now;
use beep::beep;
#[derive(Debug, Clone)]
pub struct Algorithm{
    name:String,
}

impl Algorithm {

    pub fn new() -> Algorithm{
        Algorithm{name:"Test".to_owned()}
    }

    pub fn sort<'a>(&'a self, list: &'a mut GuiVec) -> impl Generator<Yield=GuiVec, Return=()> +'a{
        move ||{
            yield list.clone();

            for index in 0..list.clone().len(){
                let mut j = index;
                while j>0 && list.get(j-1).position > list.get(j).position{
                    list.swap(j, j-1);
                    yield list.clone();
                    j = j-1;
                }
            }
        }
    }

}