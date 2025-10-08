use crate::input::{InputAction, InputHandler};
use crate::render::{Canvas, RenderEngine};
use crate::result::Result;
use crate::time::TimeController;
use crate::zoom::{Position, ZoomLevel, ZoomManager};
use std::thread::sleep;

use super::WorldState;

struct RenderState<'a> {
    fps: f32,
    show_help: bool,
    time_str: String,
    is_paused: bool,
    speed: f64,
    zoom_level: ZoomLevel,
    position: Position,
    tick_count: u64,
    entity_name: String,
    entity_count: usize,
    _phantom: std::marker::PhantomData<&'a ()>,
}

pub struct GameLoop<'a> {
    render_engine: RenderEngine<'a>,
    time_controller: TimeController,
    zoom_manager: ZoomManager,
    world_state: WorldState,
    input_handler: InputHandler,
}

impl<'a> GameLoop<'a> {
    pub fn new(render_engine: RenderEngine<'a>) -> Self {
        Self {
            render_engine,
            time_controller: TimeController::new(30),
            zoom_manager: ZoomManager::new(),
            world_state: WorldState::new(),
            input_handler: InputHandler::new(),
        }
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            if self.handle_input()? {
                break;
            }

            if !self.time_controller.is_paused() {
                self.update();
            }

            self.render()?;

            sleep(self.time_controller.target_frame_duration());
        }

        self.render_engine.exit()?;
        Ok(())
    }

    fn handle_input(&mut self) -> Result<bool> {
        let action = self.input_handler.poll()?;

        match action {
            InputAction::Quit => return Ok(true),
            InputAction::TogglePause => self.time_controller.toggle_pause(),
            InputAction::IncreaseSpeed => self.time_controller.increase_speed(),
            InputAction::DecreaseSpeed => self.time_controller.decrease_speed(),
            InputAction::ZoomIn => {
                self.zoom_manager.zoom_in();
            }
            InputAction::ZoomOut => {
                self.zoom_manager.zoom_out();
            }
            InputAction::ToggleHelp | InputAction::None => {}
        }

        Ok(false)
    }

    fn update(&mut self) {
        let delta = self.time_controller.step();
        self.world_state.update(delta);
    }

    fn render(&mut self) -> Result<()> {
        self.render_engine.begin_frame()?;

        let zoom_level = self.zoom_manager.current_level();
        let state = RenderState {
            fps: self.render_engine.fps(),
            show_help: self.input_handler.is_help_visible(),
            time_str: self.time_controller.format_time(),
            is_paused: self.time_controller.is_paused(),
            speed: self.time_controller.speed_multiplier(),
            zoom_level,
            position: *self.zoom_manager.position(),
            tick_count: self.world_state.tick_count(),
            entity_name: self.world_state.get_current_entity_name(zoom_level),
            entity_count: self.world_state.entity_count(),
            _phantom: std::marker::PhantomData,
        };

        Self::draw_game(self.render_engine.canvas_mut(), &state);

        self.render_engine.end_frame()?;
        Ok(())
    }

    fn draw_game(canvas: &mut Canvas, state: &RenderState) {
        let (width, height) = (canvas.width(), canvas.height());

        canvas.draw_box(0, 0, width, 3);
        let pause_indicator = if state.is_paused {
            "[PAUSED]"
        } else {
            "[PLAYING]"
        };
        let status_text = format!(
            "Econogenesis v0.1.0 | {} | {} {:.1}x | FPS: {:.1}",
            state.zoom_level, pause_indicator, state.speed, state.fps
        );
        canvas.draw_text(2, 1, &status_text);

        let content_y = 4;
        let content_height = height - content_y - 2;
        canvas.draw_box(0, content_y, width, content_height);

        if state.show_help {
            Self::draw_help_overlay(canvas, content_y);
        } else {
            Self::draw_zoom_view(canvas, content_y, state.zoom_level);

            let info_y = content_y + 2;
            canvas.draw_text(2, info_y, &format!("Simulation Time: {}", state.time_str));
            canvas.draw_text(2, info_y + 1, &format!("Location: {}", state.entity_name));
            let coords = state.position.coords_for_level(state.zoom_level);
            canvas.draw_text(
                2,
                info_y + 2,
                &format!(
                    "Position: ({:.1}, {:.1}, {:.1})",
                    coords.0, coords.1, coords.2
                ),
            );
            canvas.draw_text(
                2,
                info_y + 3,
                &format!(
                    "World: {} entities | Tick: {}",
                    state.entity_count, state.tick_count
                ),
            );
        }

        let status_y = height - 2;
        canvas.draw_box(0, status_y, width, 2);
        let controls_text = "[SPACE] Play/Pause | [+/-] Speed | [Z/X] Zoom | [H/?] Help | [Q] Quit";
        canvas.draw_text(2, status_y + 1, controls_text);
    }

    fn draw_help_overlay(canvas: &mut Canvas, content_y: u16) {
        let help_y = content_y + 2;

        canvas.draw_text(2, help_y, "╔══════════════════════════════════════╗");
        canvas.draw_text(2, help_y + 1, "║          KEYBOARD CONTROLS           ║");
        canvas.draw_text(2, help_y + 2, "╠══════════════════════════════════════╣");
        canvas.draw_text(2, help_y + 3, "║  SPACE     Play/Pause simulation     ║");
        canvas.draw_text(2, help_y + 4, "║  +/=       Increase time speed       ║");
        canvas.draw_text(2, help_y + 5, "║  -/_       Decrease time speed       ║");
        canvas.draw_text(2, help_y + 6, "║  Z         Zoom in                   ║");
        canvas.draw_text(2, help_y + 7, "║  X         Zoom out                  ║");
        canvas.draw_text(2, help_y + 8, "║  H/?       Toggle this help          ║");
        canvas.draw_text(2, help_y + 9, "║  Q/ESC     Quit application          ║");
        canvas.draw_text(2, help_y + 10, "╠══════════════════════════════════════╣");
        canvas.draw_text(2, help_y + 11, "║  Press H or ? to close this help     ║");
        canvas.draw_text(2, help_y + 12, "╚══════════════════════════════════════╝");
    }

    fn draw_zoom_view(canvas: &mut Canvas, content_y: u16, level: ZoomLevel) {
        let view_y = content_y + 6;

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
}
