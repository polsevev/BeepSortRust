use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, draw_text, get_fps, get_time, next_frame, screen_width};
use macroquad::ui::root_ui;
use macroquad::window::screen_height;
use crate::Algorithm::Algorithm;
use crate::GuiHookVec::GuiVec;
pub struct State{

}

impl State{
    pub async fn runInsertSort(length:i32, mut generator:impl Generator<Yield=GuiVec, Return=()>+ std::marker::Unpin){
        let mut finished = false;

        let mut paused = false;
        let timeout = 0.0;
        let mut lasttime:f64 = 0.;
        let mut holder = GuiVec::new(screen_width(), screen_height(), length);
        while !finished{
            clear_background(WHITE);
            if get_time()-lasttime > timeout && !finished && !paused{
                match Pin::new(& mut generator).resume(()){
                    GeneratorState::Yielded(x) => {
                        holder = x;
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
            next_frame().await
        }
    }
}