use crate::Result;
use tty_interface::{Device, Interface, Position, pos};

pub struct Canvas<'a> {
    interface: Interface<'a>,
    width: u16,
    height: u16,
}

impl<'a> Canvas<'a> {
    pub fn new(device: &'a mut dyn Device) -> Result<Canvas<'a>> {
        let interface = Interface::new_alternate(device)?;

        let mut canvas = Self {
            interface,
            width: 0,
            height: 0,
        };
        canvas.update_size()?;

        Ok(canvas)
    }

    pub fn apply_staged_updates(&mut self) -> Result<()> {
        Ok(self.interface.apply()?)
    }

    pub fn exit(self) -> Result<()> {
        Ok(self.interface.exit()?)
    }

    pub fn update_size(&mut self) -> Result<()> {
        (self.width, self.height) = crossterm::terminal::size()?;
        Ok(())
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str) {
        self.interface.set(pos!(x, y), text);
    }

    pub fn draw_horizontal_line(&mut self, x: u16, y: u16, mut length: u16, ch: char) {
        let available_space = self.width - x;
        if length > available_space {
            length = available_space;
        }

        let line = ch.to_string().repeat(length as usize);
        self.interface.set(pos!(x, y), &line);
    }

    pub fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) {
        // Top border
        self.draw_text(x, y, "┌");
        self.draw_horizontal_line(x + 1, y, width - 2, '─');
        self.draw_text(x + width - 1, y, "┐");

        // Sides
        for i in 1..height - 1 {
            self.draw_text(x, y + i, "│");
            self.draw_text(x + width - 1, y + i, "│");
        }

        // Bottom border
        self.draw_text(x, y + height - 1, "└");
        self.draw_horizontal_line(x + 1, y + height - 1, width - 2, '─');
        self.draw_text(x + width - 1, y + height - 1, "┘");
    }
}
