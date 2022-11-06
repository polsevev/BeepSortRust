#![feature(generators, generator_trait)]
mod BarPlugin;
mod GuiHookVec;
mod Algorithm;
mod StateMover;


use std::pin::Pin;
use macroquad::prelude::*;
use macroquad::prelude::scene::clear;
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;
use crate::StateMover::State;
use std::ops::{Generator, GeneratorState};

const BAR_WIDTH:f32 = 10.0;
#[macroquad::main("BeepSort")]
async fn main() {
    let mut gui_vec = GuiVec::new(30);
    gui_vec.randomize();
    let mut lasttime:f64 = 0.;
    let mut holder = gui_vec.clone();
    let mut algo = Algorithm::Algorithm::new();
    let mut generator = algo.sort(&mut gui_vec);
    let mut finished = false;


    loop {
        if get_time()-lasttime > 1.0 && !finished{
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

        //clear_background(WHITE);
        holder.draw();
        next_frame().await
    }
}



