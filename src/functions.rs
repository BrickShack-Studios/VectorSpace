extern crate sdl2;

#[path = "./entity.rs"] mod entity;

use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::rect::{Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::video::{Window, WindowContext};

pub fn add_vector(v1: Vec<f32>, v2: Vec<f32>) -> Vec<f32> {
    vec![v2[0] + v1[0], v2[1] + v1[1]]
}

pub fn scalar_vector(v: Vec<f32>, m: f32) -> Vec<f32> {
    vec![m * v[0], m * v[1]]
}

pub fn flip_vector(v: Vec<f32>) -> Vec<f32> {
    scalar_vector(v, -1.0)
}

pub fn sub_vector(v1: Vec<f32>, v2: Vec<f32>) -> Vec<f32> {
    add_vector(v1, flip_vector(v2))
}

pub fn magnitude(vector : Vec<f32>) -> f32 {
    vector.into_iter().map(|x| x * x).collect::<Vec<f32>>().iter().sum::<f32>().sqrt()
}

pub fn normalize(vector: Vec<f32>) -> Vec<f32> {
    let m = magnitude(vector.to_vec());
    if m < 0.001 {
        vector
    }
    else {
        vector.into_iter().map(|x| x * (1.0/m)).collect()
    }
}

pub fn is_intersecting(x1:i32, y1:i32, x2:i32, y2:i32, w:i32, h:i32) -> bool {
    x1 >= x2 && x1 <= x2 + w && y1 >= y2 && y1 <= y2 + h
}

pub fn get_texture(texture_creator: &TextureCreator<WindowContext>, src: String) -> Result<Texture, String> {
    match texture_creator.load_texture(src) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from("Bad"))
    }
}