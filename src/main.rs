
mod BarPlugin;
mod GuiHookVec;
mod algorithm;
mod dropdown;
mod soundGenerator;
use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

use dropdown::ButtonDropDown;


use macroquad::prelude::*;

use macroquad::hash;
use macroquad::ui::root_ui;






#[macroquad::main("BeepSort")]
async fn main() {
    let mut length = 1_usize;
    let mut lengthString = "100".to_owned();

    let mut delayText = "1".to_owned();

    let mut algorithm = algorithm::Algorithm::new();
    let mut buttonDropDown = ButtonDropDown::new(&algorithm.getAlgorithms());



    loop{
        clear_background(WHITE);

        length = match lengthString.parse::<usize>(){
            Ok(a) => a,
            Err(_)=> {100}
        };

        let mut centerX = screen_width()/2.0;


        draw_text("Sorting!", centerX-170.0, screen_height()*0.1, 100.0, BLACK);
        draw_text(&get_fps().to_string(), centerX + 300., 30.0, 20.0, BLACK);
        root_ui().window(hash!(), Vec2::new(centerX - 150.0, 150.), Vec2::new(300., 45.), |ui|{
            ui.input_text(hash!(), "Delay (ms)", &mut delayText);
            ui.input_text(hash!(), "Length Of Array!", &mut lengthString);
        });
 
        let mut algo = buttonDropDown.render();
        
        if algo != ""{
            algorithm::Algorithm::run(length, 1.0, algo.to_string()).await;
        }
        next_frame().await
    }



}



