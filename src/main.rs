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
use crate::Algorithm::AlgoEnum;


#[macroquad::main("BeepSort")]
async fn main() {





    let mut finished = false;

    let mut length = 100;




    loop{
        clear_background(WHITE);
        if root_ui().button(Vec2::new(screen_width()*0.2, 50.), "Increase"){
            length += 10;
        }
        if root_ui().button(Vec2::new(screen_width()*0.3, 50.), "Decrease"){
            length -= 10;
        }
        draw_text(format!("Length: {}", length.to_string()).as_str(), screen_width()*0.01, 50., 20.0, BLACK);
        if root_ui().button(Vec2::new(screen_width()*0.01, 70.), "InsertSort"){
            State::State::runInsertSort(length,  Algorithm::Algorithm::insertSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 100.), "BogoSort"){
            State::State::runInsertSort(length,  Algorithm::Algorithm::bogoSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 130.), "BubbleSort"){
            State::State::runInsertSort(length,  Algorithm::Algorithm::bogoSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 160.), "StalinSort"){
            State::State::runInsertSort(length,  Algorithm::Algorithm::bogoSort(length)).await;
        }
        next_frame().await
    }



}



