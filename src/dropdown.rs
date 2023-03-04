use macroquad::hash;

use macroquad::{ui::root_ui, window::screen_width, prelude::Vec2};


enum Status{
    Open,
    Closed
}

pub struct ButtonDropDown{
    selected:String,
    elements:Vec<String>,
    status:Status
}

impl ButtonDropDown{
    pub fn new(elements:&Vec<String>) -> Self{
        ButtonDropDown{
            selected:elements.get(0).unwrap().clone(),
            elements:elements.clone(),
            status:Status::Closed
        }
    }
    
    pub fn render(&mut self) -> String{
        let mut algo = "";
        let location = Vec2::new((screen_width() / 2.) - 150., 200.);
        match self.status{
            
            Status::Open => {
                let size = Vec2::new(300., (self.elements.len() as f32*25.0) + 20.0);
                root_ui().window(hash!(), location, size, |ui|{
                    let mut position = Vec2::new(10.0, 10.);

                    for i in self.elements.iter(){
                        let label = format!("{}{}", i[0..1].to_string().to_uppercase(), i[1..i.len()].to_string());
                        if ui.button(position, label.as_str()){
                            self.selected = i.clone();
                            self.status = Status::Closed;
                        }
                        position.y += 25.0;
                    }
                });
            }
            Status::Closed => {
                root_ui().window(hash!(), location, Vec2::new(300., 50.), |ui|{
                    let uppercasedSelected = format!("{}{}", self.selected[0..1].to_string().to_uppercase(), self.selected[1..self.selected.len()].to_string());
                    ui.label(Vec2::new(10.0, 0.0), format!("Curent chosen algorithm: {}", uppercasedSelected).as_str());
                    if ui.button(Vec2::new(10.0, 20.0), "Open Menu!"){
                        self.status = Status::Open;
                    }
                    if ui.button(Vec2::new(200.0, 20.0), "RUN!"){
                        algo = self.selected.as_str();
                    }
                });
            },
        }

        algo.to_string()
    }
}

