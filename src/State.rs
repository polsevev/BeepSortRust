use std::ops::Generator;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use crate::Algorithm::Algorithm;
use crate::GuiHookVec::GuiVec;
#[derive(PartialEq)]
pub enum GeneratorEnum{
    InsertSort,
    BogoSort,
    StalinSort,
    BubbleSort,
}

