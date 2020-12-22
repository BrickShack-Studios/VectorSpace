extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

pub struct Controller {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shift: bool,
    space: bool
}
 
pub fn main() -> Result<(), String> {
    let mut controller = Controller {
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

    let texture = texture_creator.load_texture("./images/playerShip2_orange.png")?;

    let mut x = 0;
    let mut y = 0;

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

        if (controller.up) {
            y -= 1;
        }
        else if (controller.down) {
            y += 1;
        }

        if (controller.left) {
            x -= 1;
        }
        else if (controller.right) {
            x += 1;
        }

        canvas.copy(&texture, None, Rect::new(x, y, 35, 30))?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}