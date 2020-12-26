extern crate sdl2;

#[path = "./entity.rs"] mod entity;

use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::{Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::video::{Window, WindowContext};

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

pub fn get_texture(texture_creator: &TextureCreator<WindowContext>, src: String) -> Result<Texture, String> {
    match texture_creator.load_texture(src) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from("Bad"))
    }
}