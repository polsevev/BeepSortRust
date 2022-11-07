
use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::path::Iter;
use macroquad::color::BROWN;
use macroquad::rand::ChooseRandom;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};
use crate::BarPlugin::Bar;
use crate::Algorithm::Algorithm;



#[derive(Clone, Debug)]
pub struct GuiVec{
    list: Vec<Bar>,
    initialSize:usize,
    pub lastTime:f64,

    pub reads:i32,
    pub writes:i32,
    pub comps:i32,
    screen_height:f32,
    screen_width:f32
}

impl GuiVec{
    pub fn new(screen_width:f32, screen_height:f32,length:i32) -> Self {
        let barWidth = (screen_width/((length) as f32)) - 1_f32;
        let barHeightStep = (screen_height/((length) as f32));
        let mut list:Vec<Bar> = vec!();
        for i in 1..length+1 {
            list.push(Bar::new(barWidth, barHeightStep*(i as f32) ,i));
        }
        GuiVec{list, initialSize:length as usize, lastTime: 0.0 ,  reads:0, writes:0, comps:0, screen_height, screen_width}
    }

    pub fn draw(&self){
        let mut count = 0;
        for bar in  &self.list{
            draw_rectangle(screen_width() * ((count as f32)/(self.initialSize as f32)), (screen_height()-bar.height)-50., screen_width()/((self.len()) as f32), (screen_height()/((self.len()) as f32))*bar.position as f32 * 4., bar.color);
            count += 1;
        }
    }

    pub fn resize(&mut self, length:i32){
        self.list = GuiVec::new(self.screen_width, self.screen_height,length).list;
    }
    pub fn len(&self) -> usize{
        self.list.len()
    }

    pub fn pop(&mut self) -> Bar {
        self.list.pop().unwrap()
    }

    pub fn insert(&mut self, index:usize, element: Bar){
        self.list.insert(index, element)
    }

    pub fn delete(&mut self, index:usize){
        self.writes += 1;
        self.list.remove(index);
        self.initialSize -= 1;
    }

    pub fn swap(&mut self, index1:usize, index2:usize) -> GuiVec{
        self.writes += 2;
        self.reads += 2;
        self.list.swap(index1, index2);
        self.clone()
    }
    pub fn randomize(&mut self){
        self.list.shuffle();
    }

    pub fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    pub fn get(&mut self, i:usize)-> &Bar{
        self.reads += 1;
        self.list.get(i).unwrap()
    }
    pub fn lessThan(&mut self, a:usize, b:usize) -> bool{
        self.comps += 1;
        return self.get(a).position < self.get(b).position
    }
    pub fn isSorted(&mut self) -> bool{
        self.reads += self.len() as i32;
        let mut prev = 0;
        for bar in self.list.iter() {
            if bar.position < prev{
                return false;
            }else{
                prev = bar.position;
            }
        }
        true
    }
}

