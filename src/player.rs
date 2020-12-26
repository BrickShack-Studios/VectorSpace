extern crate sdl2;

#[path = "./entity.rs"] mod e;
#[path = "./functions.rs"] mod functions;

use sdl2::render::{Texture};
use sdl2::rect::{Rect};

pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shift: bool,
    pub space: bool
}

pub struct Player<'a> {
    pub entity: e::Entity<'a>,
    pub score: u32,
    pub lives: u8,
}

impl <'a>Player <'a> {
    pub fn new(x: f32, y: f32, w: u32, h:u32, speed:f32, texture: Texture<'a>) -> Player<'a> {
        Player {
            entity: e::Entity::new(x, y, w, h, speed, texture),            
            score: 0u32,
            lives: 3u8
        }
    }

    pub fn r#move(&mut self, controller: &Controller) {
        if controller.up {
            self.entity.velocity[1] = -1.0;
        }
        else if controller.down {
            self.entity.velocity[1] = 1.0;
        }
        else {
            self.entity.velocity[1] = 0.0;
        }

        if controller.left {
            self.entity.velocity[0] = -1.0;
        }
        else if controller.right {
            self.entity.velocity[0] = 1.0;
        }
        else {
            self.entity.velocity[0] = 0.0;
        }

        let v = functions::normalize(self.entity.velocity.to_vec());
        
        self.entity.x += self.entity.speed * v[0];
        self.entity.y += self.entity.speed * v[1];

        self.entity.set_dst();
    }
}