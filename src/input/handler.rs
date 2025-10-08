use crate::result::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Quit,
    TogglePause,
    IncreaseSpeed,
    DecreaseSpeed,
    ZoomIn,
    ZoomOut,
    ToggleHelp,
    None,
}

pub struct InputHandler {
    show_help: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self { show_help: false }
    }

    pub fn poll(&mut self) -> Result<InputAction> {
        if event::poll(Duration::ZERO)?
            && let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
        {
            let action = match code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => InputAction::Quit,
                KeyCode::Char(' ') => InputAction::TogglePause,
                KeyCode::Char('+') | KeyCode::Char('=') => InputAction::IncreaseSpeed,
                KeyCode::Char('-') | KeyCode::Char('_') => InputAction::DecreaseSpeed,
                KeyCode::Char('z') | KeyCode::Char('Z') => InputAction::ZoomIn,
                KeyCode::Char('x') | KeyCode::Char('X') => InputAction::ZoomOut,
                KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('?') => {
                    InputAction::ToggleHelp
                }
                _ => InputAction::None,
            };

            if action == InputAction::ToggleHelp {
                self.show_help = !self.show_help;
            }

            Ok(action)
        } else {
            Ok(InputAction::None)
        }
    }

    pub fn is_help_visible(&self) -> bool {
        self.show_help
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
