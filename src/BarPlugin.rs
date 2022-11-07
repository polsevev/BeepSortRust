use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::rand;

#[derive(Debug, Clone)]
pub struct Bar {
    pub width:f32,
    pub height:f32,
    pub position:i32,
    pub color:Color
}



impl Bar{
    pub fn new(width:f32, height: f32,position:i32) -> Self{

        Bar{
            width: width,
            height: height,
            position,
            color:Color::from_rgba(rand::gen_range(0, 255),rand::gen_range(0, 254),rand::gen_range(0, 255),255),
        }
    }
}
