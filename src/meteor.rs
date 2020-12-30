extern crate sdl2;

#[path = "./entity.rs"] mod e;

use sdl2::render::{Texture};
use sdl2::rect::{Rect};

pub struct Meteor<'a> {
    pub entity: e::Entity<'a>,
}

impl <'a>Meteor<'a> {
    pub fn new(x:f32, y:f32, w:u32, h:u32, speed:f32, velocity:Vec<f32>, texture:Texture<'a>) -> Meteor {
        Meteor {
            entity: e::Entity::new(x, y, w, h, speed, velocity, texture),
        }
    }

    pub fn r#move(&mut self) {
        self.entity.y += self.entity.speed * self.entity.velocity[1];
        self.entity.set_dst();
    }
    
    pub fn on_screen(&self) -> bool {
        self.entity.y + (self.entity.h as f32) < 1200.0
    }
}