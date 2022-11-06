
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

    pub fn sort<'a>(&'a self, list: &'a mut GuiVec) -> impl Generator<Yield=GuiVec, Return=()> +'a{
        move ||{
            yield list.clone();
            list.swap(1, 10);
            yield list.clone();
            list.swap(5, 15);
            yield list.clone();
            list.randomize();
            yield list.clone();
        }
    }

}