mod render;
mod result;

use render::RenderEngine;
use std::io::Stdout;
use std::time::Duration;
use std::{io::stdout, thread::sleep};

use crate::render::Canvas;
use crate::result::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut device: Stdout = stdout();
    let mut engine = RenderEngine::new(&mut device)?;

    for frame in 0..60 {
        handle_frame(&mut engine, frame)?;

        // ~30 FPS
        sleep(Duration::from_millis(33));
    }

    engine.exit()?;

    println!("Phase 1 rendering test completed successfully!");
    Ok(())
}

fn handle_frame(engine: &mut RenderEngine, frame: i32) -> Result<()> {
    engine.begin_frame()?;

    let fps = engine.fps();
    draw_game(engine.canvas_mut(), fps, frame);

    engine.end_frame()?;

    Ok(())
}

fn draw_game(canvas: &mut Canvas, fps: f32, frame: i32) {
    let (width, height) = (canvas.width(), canvas.height());

    // Draw title box
    canvas.draw_box(0, 0, width, 3);
    let status_text = format!(
        "Econogenesis v0.1.0 - Phase 1 Test | Frame: {} | FPS: {:.1}",
        frame, fps
    );
    canvas.draw_text(2, 1, &status_text);

    // Draw main content area
    let content_y = 4;
    let content_height = height - content_y + 3;
    canvas.draw_box(0, content_y, width, content_height);

    // Draw some test content
    canvas.draw_text(2, content_y + 2, "Testing Canvas Drawing:");
    canvas.draw_text(4, content_y + 4, "✓ Terminal initialization");
    canvas.draw_text(4, content_y + 5, "✓ Canvas abstraction");
    canvas.draw_text(4, content_y + 6, "✓ Box drawing");
    canvas.draw_text(4, content_y + 7, "✓ Text rendering");
    canvas.draw_text(4, content_y + 8, "✓ FPS counter");

    // Draw a simple animation
    let anim_x = 4 + (frame % 40) as u16;
    let anim_y = content_y + 10;
    canvas.draw_text(anim_x, anim_y, "●");

    // Draw status bar
    let status_y = height - 2;
    canvas.draw_box(0, status_y, width, 2);
    let shortcut_text = &format!("Terminal Size: {}x{} | Press Ctrl+C to exit", width, height);
    canvas.draw_text(2, status_y + 1, shortcut_text);
}
