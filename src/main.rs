mod render;
mod result;
mod time;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use render::RenderEngine;
use std::io::Stdout;
use std::time::Duration;
use std::{io::stdout, thread::sleep};
use time::TimeController;

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
    let mut time_controller = TimeController::new(30);

    loop {
        if handle_input(&mut time_controller)? {
            break; // Quit requested
        }

        let _delta = time_controller.step();

        handle_frame(&mut engine, &time_controller)?;

        sleep(time_controller.target_frame_duration());
    }

    engine.exit()?;

    println!("Econogenesis exited successfully!");
    Ok(())
}

fn handle_input(time_controller: &mut TimeController) -> Result<bool> {
    if event::poll(Duration::ZERO)?
        && let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    return Ok(true); // Quit
                }
                KeyCode::Char(' ') => {
                    time_controller.toggle_pause();
                }
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    time_controller.increase_speed();
                }
                KeyCode::Char('-') | KeyCode::Char('_') => {
                    time_controller.decrease_speed();
                }
                _ => {}
            }
        }

    Ok(false)
}

fn handle_frame(engine: &mut RenderEngine, time_controller: &TimeController) -> Result<()> {
    engine.begin_frame()?;

    let fps = engine.fps();
    draw_game(engine.canvas_mut(), fps, time_controller);

    engine.end_frame()?;

    Ok(())
}

fn draw_game(canvas: &mut Canvas, fps: f32, time_controller: &TimeController) {
    let (width, height) = (canvas.width(), canvas.height());

    canvas.draw_box(0, 0, width, 3);
    let pause_indicator = if time_controller.is_paused() {
        "[PAUSED]"
    } else {
        "[PLAYING]"
    };
    let status_text = format!(
        "Econogenesis v0.1.0 - Phase 2 Test | {} {:.1}x | FPS: {:.1}",
        pause_indicator,
        time_controller.speed_multiplier(),
        fps
    );
    canvas.draw_text(2, 1, &status_text);

    // Draw main content area
    let content_y = 4;
    let content_height = height - content_y + 3;
    canvas.draw_box(0, content_y, width, content_height);

    // Draw time control info
    canvas.draw_text(2, content_y + 2, "Phase 2: Time Control System");
    canvas.draw_text(
        2,
        content_y + 4,
        &format!("Simulation Time: {}", time_controller.format_time()),
    );
    canvas.draw_text(
        2,
        content_y + 5,
        &format!("Speed: {:.1}x", time_controller.speed_multiplier()),
    );
    canvas.draw_text(
        2,
        content_y + 6,
        &format!(
            "Status: {}",
            if time_controller.is_paused() {
                "Paused"
            } else {
                "Running"
            }
        ),
    );

    canvas.draw_text(2, content_y + 8, "Features:");
    canvas.draw_text(4, content_y + 9, "✓ Play/Pause control");
    canvas.draw_text(4, content_y + 10, "✓ Speed multiplier (0.1x - 50x)");
    canvas.draw_text(4, content_y + 11, "✓ Simulation time tracking");
    canvas.draw_text(4, content_y + 12, "✓ Keyboard input handling");

    // Draw a clock animation that updates with sim time
    let clock_seconds = time_controller.simulation_time().as_secs() % 60;
    let clock_char = match clock_seconds % 4 {
        0 => "🕐",
        1 => "🕓",
        2 => "🕕",
        _ => "🕘",
    };
    canvas.draw_text(
        2,
        content_y + 14,
        &format!("Clock: {} ({}s)", clock_char, clock_seconds),
    );

    // Draw status bar with controls
    let status_y = height - 2;
    canvas.draw_box(0, status_y, width, 2);
    let controls_text = "[SPACE] Play/Pause | [+/-] Speed | [Q/ESC] Quit";
    canvas.draw_text(2, status_y + 1, controls_text);
}
