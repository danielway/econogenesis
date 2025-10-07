use std::time::{Duration, Instant};
use tty_interface::Device;

use super::Canvas;
use crate::Result;

pub struct RenderEngine<'a> {
    canvas: Canvas<'a>,
    frame_count: u64,
    last_fps_update: Instant,
    current_fps: f32,
    frames_since_last_update: u32,
}

impl<'a> RenderEngine<'a> {
    pub fn new(device: &'a mut dyn Device) -> Result<RenderEngine<'a>> {
        Ok(Self {
            canvas: Canvas::new(device)?,
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0.0,
            frames_since_last_update: 0,
        })
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<'a> {
        &mut self.canvas
    }

    pub fn begin_frame(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<()> {
        self.canvas.apply_staged_updates()?;
        self.update_fps();
        Ok(())
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;
        self.frames_since_last_update += 1;

        let elapsed = self.last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.current_fps = self.frames_since_last_update as f32 / elapsed.as_secs_f32();
            self.frames_since_last_update = 0;
            self.last_fps_update = Instant::now();
        }
    }

    pub fn fps(&self) -> f32 {
        self.current_fps
    }

    pub fn exit(self) -> Result<()> {
        self.canvas.exit()
    }
}
