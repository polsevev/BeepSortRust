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
    pub fn new(position:i32) -> Self{

        Bar{
            width: 10.0,
            height: position as f32 *10.0,
            position,
            color:Color::from_rgba(rand::gen_range(0, 255),rand::gen_range(0, 254),rand::gen_range(0, 255),255),
        }
    }
}
