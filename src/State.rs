use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::time::Instant;

use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, draw_text, get_fps, get_time, next_frame, screen_width};
use macroquad::time::get_frame_time;
use macroquad::ui::{root_ui, hash};
use macroquad::window::screen_height;
use crate::Algorithm::Algorithm;
use crate::GuiHookVec::GuiVec;
pub struct State{

}

impl State{
    pub async fn runInsertSort(timeout: f64, length:i32, mut generator:impl Generator<Yield=GuiVec, Return=()>+ std::marker::Unpin){
        let mut finished = false;
        let mut paused = false;
        let mut ret = false;
        let mut lasttime:f64 = 0.;
        let mut holder = GuiVec::new(screen_width(), screen_height(), length);
        let mut speed = 1;
        let mut speedText = "1".to_owned();
        loop{
            clear_background(WHITE);
            speed = match speedText.parse::<i32>(){
                Ok(a) => a,
                Err(error)=> {1}
            };
            for _ in 0..speed{
                if get_time()-lasttime > timeout && !finished && !paused{
                    match Pin::new(& mut generator).resume(()){
                        GeneratorState::Yielded(x) => {
                            holder = x;
                        },
                        GeneratorState::Complete(x) => {
                            finished = true;
                            paused = true;
                        }
                    };
                    lasttime = get_time();
                }
            }
            holder.draw();
            draw_text(format!("Read: {}", holder.reads).as_str(), screen_width()*0.01, 20.0, 20.0, BLACK);
            draw_text(format!("Write: {}", holder.writes).as_str(), screen_width()*0.01, 40.0, 20.0, BLACK);
            draw_text(format!("Comparisons: {}", holder.comps).as_str(), screen_width()*0.01, 60.0, 20.0, BLACK);
            draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width()*0.01, 80., 20.0, BLACK);
            root_ui().window(hash!(), Vec2::new(screen_width()*0.2, 5.), Vec2::new(200., 25.), |ui|{
                ui.input_text(hash!(), "Speed", &mut speedText);
            });

            if root_ui().button(Vec2::new(screen_width()*0.01, 90.), "Pause"){
                if paused {
                    paused = false;
                }else{
                    paused = true;
                }
            }
            if root_ui().button(Vec2::new(screen_width()*0.01, 120.), "Return"){
                ret = true;
            }
            if ret{
                break;
            }
            next_frame().await
        }
    }
}