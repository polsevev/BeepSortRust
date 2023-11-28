mod BarPlugin;
mod GuiHookVec;
mod algorithm;
mod dropdown;
use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

use dropdown::ButtonDropDown;

use anyhow::Result;
use notan::draw::*;
use notan::prelude::*;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
#[notan_main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).add_config(DrawConfig).build()
}

fn draw(gfx: &mut Graphics) {
    let _ = run(gfx);
}

fn run(gfx: &mut Graphics) -> Result<()> {
    let mut rt = Runtime::new()?;
    rt.block_on(async {
        let mut draw = gfx.create_draw();
        draw.clear(Color::BLACK);
        draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
        gfx.render(&draw);
        println!("Before sleep");
        sleep(Duration::from_millis(10000)).await;
        println!("After sleep");
        draw.clear(Color::BLACK);
        draw.rect((400., 100.), (100., 100.));
        gfx.render(&draw);
    });
    Ok(())
}
