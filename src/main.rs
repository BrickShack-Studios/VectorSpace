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
use sdl2::ttf::{self, PartialRendering, Sdl2TtfContext, FontStyle};
use sdl2::pixels::{Color};
use std::time::{Instant};
use sdl2::mixer::{self, Chunk, Channel};
use rand::Rng; 

enum GameState {
    Titlescreen,
    Play,
    GameOver
}

#[derive(PartialEq)]
enum MenuState {
    Menu,
    Instructions,
    Credits
}

enum Entities <'a> {
    Player(p::Player<'a>),
    Meteor(m::Meteor<'a>)
}

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

    let audio = sdl_context.audio().unwrap();

    sdl2::mixer::open_audio(44100, mixer::AUDIO_S16LSB, mixer::DEFAULT_CHANNELS, 1024).unwrap();
    let _mixer_context = sdl2::mixer::init(mixer::InitFlag::MP3 | mixer::InitFlag::OGG).unwrap();

    sdl2::mixer::allocate_channels(4);

    let menu_music = sdl2::mixer::Music::from_file("./audio/Six_Umbrellas_-_09_-_Joker.ogg").unwrap();
    let game_music = sdl2::mixer::Music::from_file("./audio/Six_Umbrellas_-_08_-_Stockholm.ogg").unwrap();

    let player_texture = texture_creator.load_texture("./images/playerShip2_orange.png")?;
    let meteor_texture = texture_creator.load_texture("./images/meteorBrown_big4.png")?;

    let mut font_context = ttf::init().unwrap();

    let font = font_context.load_font("./font/kenvector_future_thin.ttf", 20).unwrap();
    let menu_font = font_context.load_font("./font/raidercrusader.ttf", 100).unwrap();

    let title_texture = menu_font.render("Vector Space").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let play_texture = menu_font.render("Play").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let instr_texture = menu_font.render("Instructions").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let credits_texture = menu_font.render("Credits").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();

    let movement_texture = menu_font.render("Movement: WASD / Arrow Keys").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();    
    let shoot_texture = menu_font.render("Shoot: Spacebar").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let pause_texture = menu_font.render("Pause: Escape").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();

    let dev_texture = menu_font.render("Developed by:").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let gamedev_texture = menu_font.render("Jordan McIntyre").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let art_texture = menu_font.render("Art by:").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let artist_texture = menu_font.render("Kenney.nl").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let music_texture = menu_font.render("Music by:").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let musician_texture = menu_font.render("Six Umbrellas").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let sfx_texture = menu_font.render("Sound Effects by:").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let sfxer1_texture = menu_font.render("Kenney.nl").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    let sfxer2_texture = menu_font.render("zapsplat.com").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();
    
    let back_texture = menu_font.render("Back").solid(Color::WHITE).unwrap().as_texture(&texture_creator).unwrap();

    let ship = p::Player::new(600.0, 350.0, 35u32, 30u32, 5f32, vec![0f32, 0f32], player_texture);
    let meteor = m::Meteor::new(400f32, 300f32, 64u32, 64u32, 3f32, vec![0f32, 1f32], meteor_texture);

    let mut bg_sections: Vec<m::Meteor> = Vec::new();

    let mut entities: Vec<Entities> = Vec::new();

    let mut meteor_respawn = Instant::now();

    let mut point_vector: Vec<p::Point> = Vec::new();

    let mut game_state = GameState::Titlescreen;
    let mut menu_state = MenuState::Menu;

    entities.push(Entities::Player(ship));

    println!("{}", std::env::current_dir().unwrap().display());

    for i in -1..4 {
        let src_x = rand::thread_rng().gen_range(0..847);

        let mut bg = m::Meteor::new(0.0, (i as f32) * 250.0, 1200, 256, 7.0, vec![0.0, 1.0], f::get_texture(&texture_creator, String::from("./images/background.png")).unwrap());
        
        bg.entity.set_src(src_x, 0, 1200, 256);
        bg_sections.push(bg);
    }

    'running: loop {
        canvas.clear();

        for i in 0..bg_sections.len() {
            bg_sections[i].r#move();
            bg_sections[i].entity.draw(&mut canvas);

            if !bg_sections[i].on_screen() {
                let src_x = rand::thread_rng().gen_range(0..847);
                bg_sections[i].entity.set_src(src_x, 0, 1200, 256);
                
                bg_sections[i].entity.y = -256.0;
                bg_sections[i].entity.set_dst();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
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
                Event::MouseButtonDown { x, y, .. } => {
                    match game_state {
                        GameState::Titlescreen => {
                            match menu_state {
                                MenuState::Menu => {
                                    if f::is_intersecting(x, y, 560, 540, 60, 25) {
                                        game_state = GameState::Play;
                                        mixer::Music::pause();
                                        game_music.play(1);
                                        mixer::Music::set_pos(6.9);
                                    }
                                    if f::is_intersecting(x, y, 520, 575, 150, 25) {
                                        menu_state = MenuState::Instructions;
                                    }
                                    if f::is_intersecting(x, y, 548, 600, 95, 25) {
                                        menu_state = MenuState::Credits;
                                    }
                                },
                                MenuState::Instructions => {
                                    if f::is_intersecting(x, y, 500, 560, 200, 100) {
                                        menu_state = MenuState::Menu;
                                    }
                                },
                                MenuState::Credits => {
                                    if f::is_intersecting(x, y, 500, 560, 200, 100) {
                                        menu_state = MenuState::Menu;
                                    }
                                }
                            }      
                        },
                        _ => { }
                    }
                },
                _ => { }
            }
        }
        // The rest of the game loop goes here...

        match game_state {
            GameState::Titlescreen => {  
                if !mixer::Music::is_playing() {
                    menu_music.play(1);
                }
                match menu_state {
                    MenuState::Menu => {
                        let now = Instant::now();
                        if now - meteor_respawn > Duration::from_millis(300) {
                            let size = rand::thread_rng().gen_range(36..68) as f32;
                            let x = rand::thread_rng().gen_range(0..(1200 - size as i32)) as f32;

                            let speed = rand::thread_rng().gen_range(3.0..8.0);
                
                            entities.push(Entities::Meteor(m::Meteor::new(x, -size, size as u32, size as u32, speed, vec![0f32, 1f32], f::get_texture(&texture_creator, String::from("./images/meteorBrown_big4.png")).unwrap())));
                            point_vector.push(p::Point { x: x, y: -size });
                            
                            meteor_respawn = now;
                        }
                
                        let mut c = 0;
                        for mut i in 0..entities.len() {
                            match &mut entities[i] {
                                Entities::Player(p) => {
                                    p.titlescreen_move(point_vector.to_vec());
                                    p.entity.draw(&mut canvas); 
                                },
                                Entities::Meteor(m) => {
                                    if m.on_screen() {
                                        m.r#move();
                
                                        point_vector[c].x = m.entity.x;
                                        point_vector[c].y = m.entity.y;
                
                                        m.entity.draw(&mut canvas);
                                    }
                                    c += 1;
                                }
                            }
                        }
                        canvas.copy(&title_texture, None, Rect::new(450, 450, 300, 100));
                        canvas.copy(&play_texture, None, Rect::new(570, 535, 60, 35));
                        canvas.copy(&instr_texture, None, Rect::new(525, 565, 150, 35));
                        canvas.copy(&credits_texture, None, Rect::new(555, 595, 90, 35));
                    },
                    MenuState::Instructions => {
                        canvas.copy(&movement_texture, None, Rect::new(100, 50, 1000, 100));   
                        canvas.copy(&shoot_texture, None, Rect::new(350, 150, 500, 100));
                        canvas.copy(&pause_texture, None, Rect::new(375, 250, 450, 100));
                        canvas.copy(&back_texture, None, Rect::new(500, 550, 200, 100));
                    },
                    MenuState::Credits => { 
                        canvas.copy(&dev_texture, None, Rect::new(450, 0, 300, 100));
                        canvas.copy(&gamedev_texture, None, Rect::new(475, 90, 250, 50));
                        canvas.copy(&art_texture, None, Rect::new(500, 125, 200, 100));
                        canvas.copy(&artist_texture, None, Rect::new(525, 210, 150, 50));
                        canvas.copy(&music_texture, None, Rect::new(475, 245, 250, 100));
                        canvas.copy(&musician_texture, None, Rect::new(475, 330, 250, 50));
                        canvas.copy(&sfx_texture, None, Rect::new(450, 370, 300, 100));
                        canvas.copy(&sfxer1_texture, None, Rect::new(500, 455, 200, 50));
                        canvas.copy(&sfxer2_texture, None, Rect::new(500, 495, 200, 50));
                        canvas.copy(&back_texture, None, Rect::new(500, 550, 200, 100));
                    }
                }
            }
            GameState::Play => {
                if !mixer::Music::is_playing() {
                    game_music.play(1);
                    mixer::Music::set_pos(6.9);
                }
            },
            _ => { }         
        }
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}