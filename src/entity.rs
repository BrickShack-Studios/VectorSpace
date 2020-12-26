extern crate sdl2;

use sdl2::render::{Texture, Canvas};
use sdl2::rect::{Rect};
use sdl2::video::Window;

pub struct Entity<'a> {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
    pub texture: Texture<'a>,
    pub src: Option<Rect>,
    pub dst: Option<Rect>,
    pub velocity: Vec<f32>,
    pub speed: f32
}

impl <'a>Entity<'a> {
    pub fn new(x:f32, y:f32, w:u32, h:u32, speed:f32, texture: Texture<'a>) -> Entity<'a> {
        Entity {
            src: None,
            dst: Some(Rect::new(x as i32, y as i32, w, h)),
            velocity: vec![0f32, 0f32],
            x, y, w, h, speed, texture
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(&self.texture, self.src, self.dst)?;
        Ok(())
    }
    pub fn set_dst(&mut self) {   
        if let Some(ref mut dst) = self.dst {
            dst.x = self.x as i32;
            dst.y = self.y as i32;
        }
    }
}