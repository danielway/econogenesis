mod render;
mod result;
mod time;
mod zoom;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use render::RenderEngine;
use std::io::Stdout;
use std::time::Duration;
use std::{io::stdout, thread::sleep};
use time::TimeController;
use zoom::ZoomManager;

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
    let mut zoom_manager = ZoomManager::new();

    loop {
        if handle_input(&mut time_controller, &mut zoom_manager)? {
            break;
        }

        let _delta = time_controller.step();

        handle_frame(&mut engine, &time_controller, &zoom_manager)?;

        sleep(time_controller.target_frame_duration());
    }

    engine.exit()?;

    println!("Econogenesis exited successfully!");
    Ok(())
}

fn handle_input(
    time_controller: &mut TimeController,
    zoom_manager: &mut ZoomManager,
) -> Result<bool> {
    if event::poll(Duration::ZERO)?
        && let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
    {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                return Ok(true);
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
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                zoom_manager.zoom_in();
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                zoom_manager.zoom_out();
            }
            _ => {}
        }
    }

    Ok(false)
}

fn handle_frame(
    engine: &mut RenderEngine,
    time_controller: &TimeController,
    zoom_manager: &ZoomManager,
) -> Result<()> {
    engine.begin_frame()?;

    let fps = engine.fps();
    draw_game(engine.canvas_mut(), fps, time_controller, zoom_manager);

    engine.end_frame()?;

    Ok(())
}

fn draw_game(
    canvas: &mut Canvas,
    fps: f32,
    time_controller: &TimeController,
    zoom_manager: &ZoomManager,
) {
    let (width, height) = (canvas.width(), canvas.height());

    canvas.draw_box(0, 0, width, 3);
    let pause_indicator = if time_controller.is_paused() {
        "[PAUSED]"
    } else {
        "[PLAYING]"
    };
    let status_text = format!(
        "Econogenesis v0.1.0 | {} | {} {:.1}x | FPS: {:.1}",
        zoom_manager.current_level(),
        pause_indicator,
        time_controller.speed_multiplier(),
        fps
    );
    canvas.draw_text(2, 1, &status_text);

    let content_y = 4;
    let content_height = height - content_y - 2;
    canvas.draw_box(0, content_y, width, content_height);

    draw_zoom_view(canvas, content_y, zoom_manager);

    let info_y = content_y + 2;
    canvas.draw_text(
        2,
        info_y,
        &format!("Simulation Time: {}", time_controller.format_time()),
    );
    canvas.draw_text(
        2,
        info_y + 1,
        &format!("Current Zoom: {}", zoom_manager.current_level()),
    );
    let pos = zoom_manager.position();
    let coords = pos.coords_for_level(zoom_manager.current_level());
    canvas.draw_text(
        2,
        info_y + 2,
        &format!("Position: ({:.1}, {:.1}, {:.1})", coords.0, coords.1, coords.2),
    );

    let status_y = height - 2;
    canvas.draw_box(0, status_y, width, 2);
    let controls_text = "[SPACE] Play/Pause | [+/-] Speed | [Z/X] Zoom | [Q] Quit";
    canvas.draw_text(2, status_y + 1, controls_text);
}

fn draw_zoom_view(canvas: &mut Canvas, content_y: u16, zoom_manager: &ZoomManager) {
    use zoom::ZoomLevel;

    let view_y = content_y + 6;
    let level = zoom_manager.current_level();

    match level {
        ZoomLevel::Galaxy => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║      GALAXY VIEW                   ║");
            canvas.draw_text(2, view_y + 2, "║                                    ║");
            canvas.draw_text(2, view_y + 3, "║        *   ·    *                  ║");
            canvas.draw_text(2, view_y + 4, "║    ·       ⊙        ·              ║");
            canvas.draw_text(2, view_y + 5, "║  *    ·  YOU   *    ·    *         ║");
            canvas.draw_text(2, view_y + 6, "║         *       ·                  ║");
            canvas.draw_text(2, view_y + 7, "║    ·               *    ·          ║");
            canvas.draw_text(2, view_y + 8, "║                                    ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
        ZoomLevel::SolarSystem => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║    SOLAR SYSTEM VIEW               ║");
            canvas.draw_text(2, view_y + 2, "║                                    ║");
            canvas.draw_text(2, view_y + 3, "║              ☉                     ║");
            canvas.draw_text(2, view_y + 4, "║         o                          ║");
            canvas.draw_text(2, view_y + 5, "║     o       YOU   O                ║");
            canvas.draw_text(2, view_y + 6, "║   o                    o           ║");
            canvas.draw_text(2, view_y + 7, "║                                    ║");
            canvas.draw_text(2, view_y + 8, "║                  O                 ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
        ZoomLevel::Planet => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║      PLANET VIEW                   ║");
            canvas.draw_text(2, view_y + 2, "║                                    ║");
            canvas.draw_text(2, view_y + 3, "║        ~~~~~  ~~~~                 ║");
            canvas.draw_text(2, view_y + 4, "║    ~~~~       ^^^^  ~~~            ║");
            canvas.draw_text(2, view_y + 5, "║  ~~~    ^^^^ YOU ^^^^   ~~~        ║");
            canvas.draw_text(2, view_y + 6, "║    ^^^^       ~~~~                 ║");
            canvas.draw_text(2, view_y + 7, "║       ^^^^  ~~~~~   ^^^^           ║");
            canvas.draw_text(2, view_y + 8, "║                                    ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
        ZoomLevel::Region => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║      REGION VIEW                   ║");
            canvas.draw_text(2, view_y + 2, "║                                    ║");
            canvas.draw_text(2, view_y + 3, "║   ♣  ♠  ♣                          ║");
            canvas.draw_text(2, view_y + 4, "║  ♠ ♣    ♠  ♣                       ║");
            canvas.draw_text(2, view_y + 5, "║   ♣  ♠ YOU  ♣  ♠                   ║");
            canvas.draw_text(2, view_y + 6, "║  ♠    ♣  ♠    ♣                    ║");
            canvas.draw_text(2, view_y + 7, "║   ♣  ♠    ♣  ♠                     ║");
            canvas.draw_text(2, view_y + 8, "║                                    ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
        ZoomLevel::LocalArea => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║    LOCAL AREA VIEW                 ║");
            canvas.draw_text(2, view_y + 2, "║                                    ║");
            canvas.draw_text(2, view_y + 3, "║   ▓▓▓▓     ▓▓▓                     ║");
            canvas.draw_text(2, view_y + 4, "║   ▓  ▓     ▓ ▓                     ║");
            canvas.draw_text(2, view_y + 5, "║   ▓  ▓  @ YOU                      ║");
            canvas.draw_text(2, view_y + 6, "║   ▓▓▓▓     ▓▓▓                     ║");
            canvas.draw_text(2, view_y + 7, "║            ▓ ▓                     ║");
            canvas.draw_text(2, view_y + 8, "║                                    ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
        ZoomLevel::Room => {
            canvas.draw_text(2, view_y, "╔════════════════════════════════════╗");
            canvas.draw_text(2, view_y + 1, "║       ROOM VIEW                    ║");
            canvas.draw_text(2, view_y + 2, "║  ┌──────────────────┐              ║");
            canvas.draw_text(2, view_y + 3, "║  │                  │              ║");
            canvas.draw_text(2, view_y + 4, "║  │  [Table]         │              ║");
            canvas.draw_text(2, view_y + 5, "║  │         @ YOU    │              ║");
            canvas.draw_text(2, view_y + 6, "║  │                  │              ║");
            canvas.draw_text(2, view_y + 7, "║  │      [Chair]     │              ║");
            canvas.draw_text(2, view_y + 8, "║  └──────────────────┘              ║");
            canvas.draw_text(2, view_y + 9, "╚════════════════════════════════════╝");
        }
    }
}
