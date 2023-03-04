
use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::path::Iter;
use async_trait::async_trait;
use macroquad::color::{BROWN, WHITE};
use macroquad::hash;
use macroquad::prelude::{clear_background, Vec2, BLACK};
use macroquad::rand::ChooseRandom;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::time::{get_frame_time, get_fps};
use macroquad::ui::root_ui;
use macroquad::window::{next_frame, screen_height, screen_width};
use crate::BarPlugin::Bar;
use crate::algorithm::Algorithm;



#[derive(Clone, Debug)]
pub struct GuiVec{
    pub list: Vec<Bar>,
    initialSize:usize,
    pub lastTime:f64,
    pub reads:i32,
    pub writes:i32,
    pub comps:i32,
    isPaused:bool,
    delay:f32,
    pub done:bool,
    renderSkip:i32,
    skipped:i32

}
#[async_trait]
pub trait SortingList{

    fn new(length:usize, delay:f32) -> Self;

    fn len(&self) -> usize;

    async fn swap(&mut self, index1:usize, index2:usize) -> bool;

    async fn draw(&mut self);
    
    fn randomize(&mut self);

    fn elements(&mut self) -> std::slice::Iter<'_, Bar>;

    fn get(&mut self, i:usize)-> &Bar;

    fn lessThan(&mut self, a:usize, b:usize) -> bool;


    fn lessThanEqual(&mut self, a:usize, b:usize) -> bool;

    fn isSorted(&mut self) -> bool;

    async fn set(&mut self, i:usize, elem:Bar) -> bool;

    async fn show(&mut self);

    fn getListClone(&self) -> Vec<Bar>;
}
#[async_trait]
impl SortingList for  GuiVec{
    
    fn new(length:usize, delay:f32) -> Self {
        let colorStep = 360./length as f32;
        let mut list:Vec<Bar> = vec!();
        for i in 1..length+1 {
            list.push(Bar::new(i, (colorStep*i as f32)/360.));
        }
        GuiVec{
            list, 
            initialSize:length as usize, 
            lastTime: 0.0 ,  
            reads:0, 
            writes:0, 
            comps:0, 
            isPaused:false, 
            delay, 
            done:false,
            renderSkip:1,
            skipped:0
        }
    }

    async fn draw(&mut self){
        let mut frames = 0.0;
        let mut delayText = self.delay.to_string();
        let mut renderSkipText = self.renderSkip.to_string();

        loop {

            /*
            self.checkPaused();
            if self.isPaused{
                break;
            }

             */
            if self.skipped >= self.renderSkip{
                clear_background(WHITE);

                for (count,bar) in  self.list.iter().enumerate(){
                    draw_rectangle(screen_width() * ((count as f32)/(self.initialSize as f32)),screen_height() - (screen_height()/((self.len()) as f32))*bar.position as f32 , screen_width()/((self.len()) as f32), (screen_height()/((self.len()) as f32))*bar.position as f32, bar.color);
                    
                }
    
                draw_text(&format!("FPS: {}", get_fps()), screen_width()*0.01 + 40., 80.0, 20.0, BLACK);
                draw_text(&format!("Array reads: {}", self.reads), screen_width()*0.01 + 40., 110.0, 20.0, BLACK);
                draw_text(&format!("Array writes: {}", self.writes), screen_width()*0.01 + 40., 140.0, 20.0, BLACK);
                draw_text(&format!("Comparisons: {}", self.comps), screen_width()*0.01 + 40., 170.0, 20.0, BLACK);
                
                root_ui().window(hash!(),Vec2::new(screen_width()*0.01, 5.), Vec2::new(800.0, 50.), |ui|{
                    ui.input_text(hash!(), "Delay (ms)", &mut delayText);
                    ui.input_text(hash!(), "StepsPrFrame (How many steps of the algorithm pr frame)", &mut renderSkipText);
      
                });
                
                if root_ui().button(Vec2::new(screen_width()*0.01, 60.), "Exit"){
                    self.done = true;
                    break;
                }
                if root_ui().button(Vec2::new(screen_width()*0.01, 90.), "Pause"){
                    self.isPaused = !self.isPaused;
    
                }
                self.renderSkip = match renderSkipText.parse::<i32>(){
                    Ok(a) => a,
                    Err(_) => 1
                };
    
                self.delay = match (delayText.parse::<f32>(), self.isPaused){
                    (_, true) => f32::MAX,
                    (Ok(a), false) => a,
                    (Err(_), _)=> {f32::MAX}
                };
                next_frame().await;
                self.skipped = 0;
            }else{
                self.skipped += 1;
            }

            if frames >= self.delay && !self.done{
                break;
            }


            frames += get_frame_time()* 1000.;
        }

    }


    fn len(&self) -> usize{
        self.list.len()
    }

    async fn swap(&mut self, index1:usize, index2:usize) -> bool{
        self.writes += 2;
        self.reads += 2;
        self.list.swap(index1, index2);
        self.draw().await;
        self.done
    }
    fn randomize(&mut self){
        self.list.shuffle();
    }

    fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    fn get(&mut self, i:usize)-> &Bar{
        self.reads += 1;
        self.list.get(i).unwrap()
    }
    fn lessThan(&mut self, a:usize, b:usize) -> bool{
        self.comps += 1;
        return self.get(a).position < self.get(b).position
    }
    fn lessThanEqual(&mut self, a:usize, b:usize) -> bool{
        self.comps += 1;
        return self.get(a).position <= b
    }
    fn isSorted(&mut self) -> bool{
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
    async fn set(&mut self, i:usize, elem:Bar) -> bool{

        self.writes += 1;
        self.reads += 1;
        self.list[i] = elem;
        self.draw().await;
        self.done
    
    }
    async fn show(&mut self){
        loop{
            if !self.done{
                self.draw().await
            }else{
                break
            }
        }
    }

    fn getListClone(&self) -> Vec<Bar>{
        self.list.clone()
    }

}

pub struct NonGuiVec{
    pub list: Vec<Bar>,

}
#[async_trait]
impl SortingList for  NonGuiVec{
    fn new(length:usize, delay:f32) -> Self{
        let mut list = Vec::new();
        for i in 0..(length as usize){
            list.push(Bar::new(i, i as f32))
        }
        NonGuiVec { list: list }
    }   

    fn len(&self) -> usize{
        self.list.len()
    }

    async fn swap(&mut self, index1:usize, index2:usize) -> bool{
        self.list.swap(index1, index2);
        false
    }

    async fn draw(&mut self){
        self.swap(0, 0).await;
    }
    
    fn randomize(&mut self){
        self.list.shuffle();
    }

    fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    fn get(&mut self, i:usize)-> &Bar{

        self.list.get(i).unwrap()
    }
    fn lessThan(&mut self, a:usize, b:usize) -> bool{

        return self.get(a).position < self.get(b).position
    }
    fn lessThanEqual(&mut self, a:usize, b:usize) -> bool{
        return self.get(a).position <= b
    }
    fn isSorted(&mut self) -> bool{
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
    async fn set(&mut self, i:usize, elem:Bar) -> bool{

        self.list[i] = elem;
        self.draw().await;
        false
    
    }
    async fn show(&mut self){

    }
    fn getListClone(&self) -> Vec<Bar>{
        self.list.clone()
    }

}
