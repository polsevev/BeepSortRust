#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub position: usize,
}

impl Bar {
    pub fn new(position: usize, hsl_color: f32) -> Self {
        Bar { position }
    }
}
