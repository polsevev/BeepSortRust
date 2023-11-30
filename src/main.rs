#![feature(generators, generator_trait)]
mod BarPlugin;
mod dropdown;

use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::rc::Rc;

use anyhow::Result;
use insertSortGenerator::{insert_sort, test};
use notan::draw::*;
use notan::draw::{CreateDraw, CreateFont, DrawConfig, Font};
use notan::prelude::*;
mod insertSortGenerator;
#[derive(AppState)]
struct State {
    gen: Box<dyn Unpin + Generator<Yield = Rc<RefCell<Vec<u32>>>, Return = ()>>,
    delay: f32,
    lastStepTime: f32,
    font: Font,
    next: Option<String>,
    finished: bool,
}

fn setup(gfx: &mut Graphics) -> State {
    let mut arr = (1..=100).collect::<Vec<u32>>();
    let gen = insert_sort(arr);
    State {
        gen: Box::new(gen),
        delay: 1.,
        lastStepTime: 1.,
        font: gfx
            .create_font(include_bytes!("./assets/LDF-ComicSans/LDFComicSans.ttf"))
            .unwrap(),
        next: None,
        finished: false,
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .update(update)
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}
fn update(state: &mut State) {
    if !state.finished {
        match Pin::new(&mut state.gen).resume(()) {
            GeneratorState::Yielded(a) => {
                state.next = Some(format!("{:?}", a));
                println!("We set something")
            }
            GeneratorState::Complete(_) => {
                state.finished = true;
                println!("Generator finished");
            }
        }
    };
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    let text = if state.next.is_some() {
        state.next.clone().unwrap()
    } else {
        String::new()
    };
    draw.text(&state.font, &text)
        .position(100., 100.)
        .size(60.)
        .color(Color::WHITE)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
