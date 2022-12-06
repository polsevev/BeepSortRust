
use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::path::Iter;
use macroquad::color::{BROWN, WHITE};
use macroquad::hash;
use macroquad::prelude::{clear_background, Vec2};
use macroquad::rand::ChooseRandom;
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_frame_time;
use macroquad::ui::root_ui;
use macroquad::window::{next_frame, screen_height, screen_width};
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
    screen_width:f32,
    isPaused:bool,
    delay:f32

}

impl GuiVec{
    pub fn new(screen_width:f32, screen_height:f32,length:i32, delay:f32) -> Self {
        let colorStep = 360./length as f32;
        let mut list:Vec<Bar> = vec!();
        for i in 1..length+1 {
            list.push(Bar::new(i, (colorStep*i as f32)/360.));
        }
        GuiVec{list, initialSize:length as usize, lastTime: 0.0 ,  reads:0, writes:0, comps:0, screen_height, screen_width, isPaused:false, delay}
    }

    pub async fn draw(&mut self){
        let mut frames = 0.0;
        let mut delayText = self.delay.to_string();

        loop {
            /*
            self.checkPaused();
            if self.isPaused{
                break;
            }

             */

            clear_background(WHITE);
            let mut count = 0;
            for bar in  &self.list{
                draw_rectangle(screen_width() * ((count as f32)/(self.initialSize as f32)),screen_height() - (screen_height()/((self.len()) as f32))*bar.position as f32 , screen_width()/((self.len()) as f32), (screen_height()/((self.len()) as f32))*bar.position as f32, bar.color);
                count += 1;
            }

            root_ui().window(hash!(), Vec2::new(screen_width()*0.2, 5.), Vec2::new(200., 25.), |ui|{
                ui.input_text(hash!(), "Delay (ms)", &mut delayText);
            });
            self.delay = match delayText.parse::<f32>(){
                Ok(a) => a,
                Err(error)=> {1.0}
            };

            next_frame().await;
            if frames >= self.delay{
                break;
            }
            frames += get_frame_time()*1000.0;
        }

    }

    fn checkPaused(&mut self){
        if root_ui().button(Vec2::new(screen_width()*0.01, 90.), "Pause"){
            if self.isPaused {
                self.isPaused = false;
            }else{
                self.isPaused = true;
            }
        }
    }

    pub fn resize(&mut self, length:i32){
        self.list = GuiVec::new(self.screen_width, self.screen_height,length, self.delay).list;
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

    pub async fn swap(&mut self, index1:usize, index2:usize){
        self.writes += 2;
        self.reads += 2;
        self.list.swap(index1, index2);
        self.draw().await;
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

