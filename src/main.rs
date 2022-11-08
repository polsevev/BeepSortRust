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
use std::process::id;
use macroquad::ui::root_ui;
use crate::Algorithm::AlgoEnum;


#[macroquad::main("BeepSort")]
async fn main() {
    let mut length = 100;

    let mut delay = 0.;

    loop{
        clear_background(WHITE);
        draw_text("Sorting!", screen_width()*0.3, screen_height()*0.5, 100.0, GREEN);
        draw_text(format!("Length: {}", length.to_string()).as_str(), screen_width()*0.83, 30., 20.0, BLACK);
        if root_ui().button(Vec2::new(screen_width()*0.9, 50.), "+10"){
            length += 10;
        }
        if root_ui().button(Vec2::new(screen_width()*0.87, 50.), "+1"){
            length += 1;
        }
        if root_ui().button(Vec2::new(screen_width()*0.8, 50.), "-10"){
            length -= 10;
        }
        if root_ui().button(Vec2::new(screen_width()*0.84, 50.), "-1"){
            length -= 1;
        }


        if root_ui().button(Vec2::new(screen_width()*0.01, 70.), "InsertSort"){
            State::State::runInsertSort(delay,length,  Algorithm::Algorithm::insertSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 100.), "BogoSort"){
            State::State::runInsertSort(delay,length,  Algorithm::Algorithm::bogoSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 130.), "BubbleSort"){
            State::State::runInsertSort(delay, length,  Algorithm::Algorithm::bubbleSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 160.), "StalinSort"){
            State::State::runInsertSort(delay, length,  Algorithm::Algorithm::stalinSort(length)).await;
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 190.), "CoctailShaker"){
            State::State::runInsertSort(delay, length,  Algorithm::Algorithm::cocktailShaker(length)).await;
        }

        next_frame().await
    }



}



