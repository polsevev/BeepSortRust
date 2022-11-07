#![feature(generators, generator_trait, type_alias_impl_trait)]
mod BarPlugin;
mod GuiHookVec;
mod Algorithm;
mod State;


use std::pin::Pin;
use macroquad::prelude::*;
use macroquad::prelude::scene::clear;
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;

use std::ops::{Deref, Generator, GeneratorState};
use macroquad::ui::root_ui;
use crate::State::GeneratorEnum;

#[macroquad::main("BeepSort")]
async fn main() {

    let mut length = 100;
    let mut lasttime:f64 = 0.;
    let mut holder = GuiVec::new(screen_width(), screen_height(), length);

    let mut finished = false;
    let timeout = 0.0;
    let mut paused = false;
    let mut reset = false;


    let mut algoChoise = GeneratorEnum::InsertSort;

    loop{
        let mut generator = Algorithm::Algorithm::insertSort(length);

        while !reset {
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
            draw_text(format!("Read: {}", holder.reads).as_str(), screen_width()*0.01, 20.0, 20.0, BLACK);
            draw_text(format!("Write: {}", holder.writes).as_str(), screen_width()*0.01, 40.0, 20.0, BLACK);
            draw_text(format!("Comparisons: {}", holder.comps).as_str(), screen_width()*0.01, 60.0, 20.0, BLACK);
            draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width()*0.01, 80., 20.0, BLACK);

            if root_ui().button(Vec2::new(screen_width()*0.01, 70.), "Pause"){
                if paused {
                    paused = false;
                }else{
                    paused = true;
                }
            }
            if root_ui().button(Vec2::new(screen_width()*0.01, 95.), "Reset!"){
                reset = true;
            }
            next_frame().await
        }
        algoChoise = GeneratorEnum::StalinSort;
        reset = false;
        finished = false;
    }



}



