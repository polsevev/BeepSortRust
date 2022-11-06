
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
    algo:Algorithm
}

impl GuiVec{
    pub fn new(length:i32) -> Self {
        let mut list:Vec<Bar> = vec!();
        for i in 0..length {
            list.push(Bar::new(i));
        }
        GuiVec{list, initialSize:length as usize, lastTime: 0.0 , algo:Algorithm::new()}
    }

    pub fn draw(&self){
        let mut count = 0;
        for bar in  &self.list{
            draw_rectangle(screen_width() * ((count as f32)/(self.initialSize as f32)), screen_height()-(200.+ (bar.position as f32 * 10.0)), bar.width, bar.height, BROWN);
            count += 1;
        }
    }

    pub fn resize(&mut self, length:i32){
        self.list = GuiVec::new(length).list;
    }
    pub fn len(self) -> usize{
        self.list.len()
    }
    pub async fn push(&mut self){
        self.list.push(Bar::new(self.initialSize as i32));
        self.initialSize += 1;
    }

    pub fn pop(&mut self) -> Bar {
        self.list.pop().unwrap()
    }

    pub fn insert(&mut self, index:usize, element: Bar){
        self.list.insert(index, element)
    }

    pub async fn delete(&mut self, index:usize){
        self.list.remove(index);
        self.initialSize -= 1;
    }

    pub fn swap(&mut self, index1:usize, index2:usize){
        self.list.swap(index1, index2);
    }
    pub fn randomize(&mut self){
        self.list.shuffle();
    }

    pub fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    pub fn get(&self, i:usize)-> &Bar{
        self.list.get(i).unwrap()
    }
}

