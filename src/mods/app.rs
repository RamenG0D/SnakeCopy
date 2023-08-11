use sdl2::{self, pixels::Color, video::Window, render::Canvas, EventPump, event::Event::*, keyboard::Keycode };
use std::collections::HashMap;

fn init() -> (Canvas<Window>, EventPump, Keys) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
 
    let window = 
        video
        .window("Snake", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().accelerated().build().unwrap();
    let pump = sdl.event_pump().unwrap();

    let key_map = HashMap::from([
        ("w".to_owned(), Key::new()),
        ("a".to_owned(), Key::new()),
        ("s".to_owned(), Key::new()),
        ("d".to_owned(), Key::new()),
        ("space".to_owned(), Key::new())
    ]);

    let keys = Keys::new(key_map);

    (canvas, pump, keys)
}

pub struct App {
    canvas: Canvas<Window>,
    pump: EventPump,
    keys: Keys,
    background_color: Color
}

pub struct Keys {
    keys: HashMap<String, Key>
}

impl Keys {

    fn new(km: HashMap<String, Key>) -> Self {
        Self { keys: km }
    }

    pub fn get(&mut self, key: String) -> Option<&mut Key> {
        return self.keys.get_mut(&key);
    }

}

impl App {
    pub fn new(background_color: Color) -> Self {
        let app = init();
        Self { canvas: app.0, pump: app.1, keys: app.2, background_color }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.canvas.window_mut().set_title(title).unwrap();
        return self;
    }

    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.canvas.window_mut().set_size(width, height).unwrap();
        return self;
    }

    pub fn get_canvas(&mut self) -> &mut Canvas<Window> {
        return &mut self.canvas;
    }

    pub fn app_loop(&mut self) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                Quit { timestamp } => {
                    println!("Time of Exit => {}", timestamp);
                    return true;
                },
                KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Escape => {
                            return true;
                        },
                        Keycode::W => {
                            self.keys.get("w".to_owned()).unwrap().pressed = true;
                        },
                        Keycode::A => {
                            self.keys.get("a".to_owned()).unwrap().pressed = true;
                        },
                        Keycode::S => {
                            self.keys.get("s".to_owned()).unwrap().pressed = true;
                        },
                        Keycode::D => {
                            self.keys.get("d".to_owned()).unwrap().pressed = true;
                        },
                        Keycode::Space => {
                            self.keys.get("space".to_owned()).unwrap().pressed = true;
                        },
                        _ => {}
                    }
                }
                KeyUp { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::W => {
                            self.keys.get("w".to_owned()).unwrap().pressed = false;
                        },
                        Keycode::A => {
                            self.keys.get("a".to_owned()).unwrap().pressed = false;
                        },
                        Keycode::S => {
                            self.keys.get("s".to_owned()).unwrap().pressed = false;
                        },
                        Keycode::D => {
                            self.keys.get("d".to_owned()).unwrap().pressed = false;
                        },
                        Keycode::Space => {
                            self.keys.get("space".to_owned()).unwrap().pressed = false;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        self.clear_screen(self.background_color);
        return false;
    }

    fn clear_screen(&mut self, refresh_color: Color) {
        self.canvas.set_draw_color(refresh_color);
        self.canvas.clear();
    }

    pub fn key_pressed(&mut self, k: &str) -> bool {
        return self.keys.get(k.to_owned()).unwrap().pressed;
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

pub struct Key {
    pressed: bool
}

impl Key {
    fn new() -> Self {
        Self { pressed: false }
    }
}