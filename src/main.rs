mod game;
mod input;
mod render;
mod result;
mod time;
mod zoom;

use game::GameLoop;
use render::RenderEngine;
use result::Result;
use std::io::stdout;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut device = stdout();
    let engine = RenderEngine::new(&mut device)?;
    let game_loop = GameLoop::new(engine);

    game_loop.run()?;

    println!("Econogenesis exited successfully!");
    Ok(())
}
