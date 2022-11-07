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


const BAR_WIDTH:f32 = 10.0;
#[macroquad::main("BeepSort")]
async fn main() {
    let mut gui_vec = GuiVec::new(50);
    gui_vec.randomize();
    let mut lasttime:f64 = 0.;
    let mut holder = gui_vec.clone();
    let mut generator = Algorithm::Algorithm::insertSort(&mut gui_vec);
    let mut finished = false;
    let timeout = 0.000001;

    loop {

        clear_background(WHITE);
        if get_time()-lasttime > timeout && !finished{
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
        draw_text(format!("Read: {}", holder.reads).as_str(), screen_width()*0.1, screen_height()-100.0, 20.0, BLACK);
        draw_text(format!("Write: {}", holder.writes).as_str(), screen_width()*0.1, screen_height()-80.0, 20.0, BLACK);
        draw_text(format!("Comparisons: {}", holder.comps).as_str(), screen_width()*0.1, screen_height()-60.0, 20.0, BLACK);
        draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width()*0.1, screen_height()-40., 20.0, BLACK);
        holder.draw();
        next_frame().await
    }
}



