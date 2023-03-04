
use macroquad::audio::PlaySoundParams;
use macroquad::audio::Sound;
use macroquad::audio::play_sound;
use macroquad::audio::play_sound_once;
use macroquad::color;
use macroquad::color_u8;
use macroquad::rand;

use crate::soundGenerator;

#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub position:usize,
    pub color:color::Color,
}



impl Bar{
    pub fn new(position:usize, hsl_color:f32) -> Self{
        Bar{
            position,
            color: color::hsl_to_rgb((hsl_color as f32) , 1.0, 0.5),
        }
    }

}
