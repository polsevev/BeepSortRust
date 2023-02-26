
mod BarPlugin;
mod GuiHookVec;
mod Algorithm;

use macroquad::prelude::*;
use macroquad::prelude::scene::clear;
use crate::BarPlugin::Bar;
use crate::GuiHookVec::GuiVec;

use macroquad::hash;
use macroquad::ui::root_ui;


#[macroquad::main("BeepSort")]
async fn main() {
    let mut length = 1;
    let mut lengthString = "100".to_owned();
    let mut delay = 0.;
    let mut delayText = "0.0".to_owned();
    loop{
        clear_background(WHITE);

        delay = match delayText.parse::<f64>(){
            Ok(a) => a/1000.,
            Err(error)=> {0.1}
        };
        length = match lengthString.parse::<i32>(){
            Ok(a) => a,
            Err(error)=> {100}
        };

        draw_text("Sorting! HAHHAA", screen_width()*0.3, screen_height()*0.5, 100.0, GREEN);
        draw_text(format!("Length: {}", length.to_string()).as_str(), screen_width()*0.83, 30., 20.0, BLACK);
        draw_text(&get_fps().to_string(), screen_width()*0.7, 30.0, 20.0, BLACK);
        root_ui().window(hash!(), Vec2::new(screen_width()*0.01, 45.), Vec2::new(250., 50.), |ui|{
            ui.input_text(hash!(), "Delay (ms)", &mut delayText);
            ui.input_text(hash!(), "Length Of Array!", &mut lengthString);
        });

        if root_ui().button(Vec2::new(screen_width()*0.01, 100.), "RUN!"){
            //State::State::runInsertSort(delay,length).await;
            Algorithm::Algorithm::insertSort(length, 1.0).await;
        }
        next_frame().await
    }



}



