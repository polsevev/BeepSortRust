enum Status {
    Open,
    Closed,
}

pub struct ButtonDropDown {
    selected: String,
    elements: Vec<String>,
    status: Status,
}

impl ButtonDropDown {
    pub fn new(elements: &Vec<String>) -> Self {
        ButtonDropDown {
            selected: elements.get(0).unwrap().clone(),
            elements: elements.clone(),
            status: Status::Closed,
        }
    }

    pub fn render(&mut self) -> String {
        "LOL".to_string()
    }
}
