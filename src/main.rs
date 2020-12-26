extern crate sdl2; 

#[path = "./functions.rs"] mod f;
#[path = "./entity.rs"] mod e;
#[path = "./player.rs"] mod p;
#[path = "./meteor.rs"] mod m;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use sdl2::rect::{Rect};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::pixels::{PixelFormatEnum};

pub fn main() -> Result<(), String> {
    let mut controller = p::Controller {
        up: false,
        down: false,
        left: false,
        right: false,
        shift: false,
        space: false
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
 
    let window = video_subsystem.window("Vector Space", 1200, 650)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();

    let player_texture = texture_creator.load_texture("../images/playerShip2_orange.png")?;
    let meteor_texture = texture_creator.load_texture("../images/meteorBrown_big4.png")?;

    let mut ship = p::Player::new(0f32, 0f32, 35u32, 30u32, 5f32, f::get_texture(&texture_creator, String::from("../images/playerShip2_orange.png")).unwrap());
    let mut meteor = m::Meteor::new(400f32, 300f32, 64u32, 64u32, 3f32, meteor_texture);

    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} |
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    controller.up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    controller.left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } |
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    controller.down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    controller.right = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } |
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    controller.up = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } |
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    controller.left = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } |
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    controller.down = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } |
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    controller.right = false;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ship.r#move(&controller);
        ship.entity.draw(&mut canvas);

        meteor.entity.draw(&mut canvas);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}