
mod BarPlugin;
mod GuiHookVec;
mod algorithm;

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
    let mut delayText = "1".to_owned();
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

        draw_text("Sorting!", screen_width()*0.3, screen_height()*0.1, 100.0, GREEN);
        draw_text(format!("Length: {}", length.to_string()).as_str(), screen_width()*0.83, 30., 20.0, BLACK);
        draw_text(&get_fps().to_string(), screen_width()*0.7, 30.0, 20.0, BLACK);
        root_ui().window(hash!(), Vec2::new(screen_width()*0.01, 45.), Vec2::new(250., 50.), |ui|{
            ui.input_text(hash!(), "Delay (ms)", &mut delayText);
            ui.input_text(hash!(), "Length Of Array!", &mut lengthString);
        });
        let mut algo = "";
        if root_ui().button(Vec2::new(screen_width()*0.01, 100.), "Run InsertSort!"){
            algo = "insertSort";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 130.), "Run BubbleSort!"){
            algo = "bubbleSort";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 160.), "Run BinaryHeapSort!"){
            algo = "binaryHeap";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 190.), "Run CoctailShakerSort!"){
            algo = "cocktailShaker";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 220.), "Run BogoSort!"){
            algo = "bogoSort";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 250.), "Run QuickSort!"){
            algo = "quickSort";
        }
        if root_ui().button(Vec2::new(screen_width()*0.01, 280.), "Run RadixSort!"){
            algo = "radixSort";
        }


        if algo != ""{
            algorithm::Algorithm::run(length, 1.0, algo.to_string()).await;
        }
        next_frame().await
    }



}



