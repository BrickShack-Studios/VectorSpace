extern crate sdl2;

#[path = "./entity.rs"] mod e;
#[path = "./functions.rs"] mod f;

use sdl2::render::{Texture};
use sdl2::rect::{Rect};
use std::time::{Instant};

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

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl <'a>Player <'a> {
    pub fn new(x: f32, y: f32, w: u32, h:u32, speed:f32, velocity:Vec<f32>, texture: Texture<'a>) -> Player<'a> {
        Player {
            entity: e::Entity::new(x, y, w, h, speed, velocity, texture),            
            score: 0u32,
            lives: 3u8
        }
    }
    
    pub fn valid_x_move(&self) -> bool {
        let x = self.entity.x + (self.entity.speed * self.entity.velocity[0]);
        if x >= 0.0 && x + self.entity.w as f32 <= 1200.0 {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn valid_y_move(&self) -> bool {
        let y = self.entity.y + (self.entity.speed * self.entity.velocity[1]);
        if y >= 0.0 && y + self.entity.h as f32 <= 650.0 {
            return true;
        }
        else {
            return false;
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

        let b1 = self.valid_x_move();
        let b2 = self.valid_y_move();

        let v = f::normalize(self.entity.velocity.to_vec());

        if b1 && b2 {
            self.entity.x += self.entity.speed * v[0];
            self.entity.y += self.entity.speed * v[1];
        }
        else {
            if b1 {
                self.entity.x += self.entity.speed * v[0];
            }
            else {
                if self.entity.velocity[0] == 1.0 {
                    self.entity.x = 1200.0 - (self.entity.w as f32);
                }
                else {
                    self.entity.x = 0.0;
                }
            }
            if b2 {
                self.entity.y += self.entity.speed * v[1];
            }
            else {
                if self.entity.velocity[1] == 1.0 {
                    self.entity.y = 650.0 - (self.entity.h as f32);
                }
                else {
                    self.entity.y = 0.0;
                }
            }
        }
        
        self.entity.set_dst();
    }

    pub fn titlescreen_move(&mut self, meteors: Vec<Point>) {

        let point_bias = 0.1;
        let meteor_bias = 1000000.0;
        let falloff = 2.4;

        let now = Instant::now().elapsed().as_millis() as f32;

        let position = vec![0f32, 0f32];
        let point = vec![500.0 + (100.0 * (now / 1000.0).cos()), 350.0 + (100.0 * (now / 1000.0).sin())];

        let player_to_point = f::sub_vector(point, vec![self.entity
            .x, self.entity.y]);
        let to_point_velocity = f::scalar_vector(player_to_point, point_bias);

        let mut position = f::add_vector(position, to_point_velocity);

        for i in 0..meteors.len() {
            let player_to_meteor = f::sub_vector(vec![meteors[i].x, meteors[i].y], vec![self.entity.x, self.entity.y]);
            let meteor_distance = f::magnitude(player_to_meteor.to_vec());
            let away_from_meteor = f::flip_vector(f::normalize(player_to_meteor.to_vec()));
            let away_position = f::scalar_vector(away_from_meteor.to_vec(), meteor_bias / meteor_distance.powf(falloff));

            position = f::add_vector(position, away_position.to_vec());
        }

        position = f::add_vector(vec![self.entity.x, self.entity.y], position);

        if position[0] >= 0.0 && position[0] + (self.entity.w as f32) <= 1200.0 {
            self.entity.x = position[0];
        }
        else {
            if position[0] < 0.0 {
                self.entity.x = 0.0;
            }
            else {
                self.entity.x = 1200.0 - (self.entity.w as f32);
            }
        }
       
        if position[1] >= 0.0 && position[1] + (self.entity.h as f32) <= 650.0 {
            self.entity.y = position[1];
        }
        else {
            if position[0] < 0.0 {
                self.entity.y = 0.0;
            }
            else {
                self.entity.y = 650.0 - (self.entity.h as f32);
            }
        }

        self.entity.set_dst();
    }
}