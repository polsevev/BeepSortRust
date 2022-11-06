
#[derive(Debug, Clone)]
pub struct Bar {
    pub width:f32,
    pub height:f32,
    pub position:i32
}



impl Bar{
    pub fn new(position:i32) -> Self{
        Bar{
            width: 10.0,
            height: position as f32 *10.0,
            position
        }
    }
}
