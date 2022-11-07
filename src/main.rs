#![feature(generators, generator_trait)]
mod BarPlugin;
mod GuiHookVec;
mod Algorithm;


use std::pin::Pin;
use macroquad::prelude::*;
use macroquad::prelude::scene::clear;
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;

use std::ops::{Generator, GeneratorState};
use macroquad::ui::root_ui;
use crate::Algorithm::Algo;


const BAR_WIDTH:f32 = 10.0;
#[macroquad::main("BeepSort")]
async fn main() {
    let mut gui_vec = GuiVec::new(screen_width(), screen_height(), 200);
    gui_vec.randomize();
    let mut lasttime:f64 = 0.;
    let mut holder = gui_vec.clone();
    let mut generator = Algorithm::Algorithm::bubbleSort(&mut gui_vec);
    let mut finished = false;
    let timeout = 0.000001;
    let mut paused = false;
    loop {
        clear_background(WHITE);
        if get_time()-lasttime > timeout && !finished && !paused{
            match Pin::new(& mut generator).resume(()){
                GeneratorState::Yielded(x) => {

                    holder = x.clone();
                },
                GeneratorState::Complete(x) => {
                    finished = true;
                }
            };
            lasttime = get_time();
        }
        holder.draw();
        draw_text(format!("Read: {}", holder.reads).as_str(), screen_width()*0.01, 0.0, 20.0, BLACK);
        draw_text(format!("Write: {}", holder.writes).as_str(), screen_width()*0.01, 20.0, 20.0, BLACK);
        draw_text(format!("Comparisons: {}", holder.comps).as_str(), screen_width()*0.01, 40.0, 20.0, BLACK);
        draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width()*0.01, 60., 20.0, BLACK);

        if root_ui().button(Vec2::new(screen_width()*0.01, 70.), "Pause"){
            if paused {
                paused = false;
            }else{
                paused = true;
            }
        }
        next_frame().await
    }
}



