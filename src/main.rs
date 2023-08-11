use std::time::Duration;

use mods::app::App;
use sdl2::pixels::Color;
use snake::{Snake, Position, Direction, Apple, check_collisions, apple_check, display_death};

pub mod mods;

pub const UNIT_SIZE: i16 = 20;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let mut ai_enabled: bool = false;

    for x in &args {
        if x == "--enable-ai" || x == "-ai"
        {
            ai_enabled = true;
        }
    }

    let mut snake = Snake::default();
    let mut apple = Apple::default();

    let mut app = App::new(Color::BLACK);
    let mut fps = 0;

    let mut death = false;
    let mut score = 0;

    let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;
    const PATH: &'static str = "ChunkFive-Regular.otf"; const P_SIZE: u16 = 128;
    let mut font = ttf.load_font(PATH, P_SIZE)?;

    'running: loop {
        if app.app_loop() { break 'running; }

        if fps < 30 {
            if death {
                display_death(app.get_canvas(), &mut font, &score)?;
                if app.key_pressed("space") {
                    restart(&mut score, &mut snake, &mut apple,&mut death);
                } else {
                    continue;
                }
            }

            let canvas = app.get_canvas();
            apple.draw_apple(canvas); snake.move_snake(&ai_enabled); snake.draw_snake(canvas);

            if !ai_enabled {
                if app.key_pressed("w") && snake.dir != Direction::Down {
                    snake.dir = Direction::Up;
                } else if app.key_pressed("a") && snake.dir != Direction::Right {
                    snake.dir = Direction::Left;
                } else if app.key_pressed("s") && snake.dir != Direction::Up {
                    snake.dir = Direction::Down;
                } else if app.key_pressed("d") && snake.dir != Direction::Left {
                    snake.dir = Direction::Right;
                }
            }

            if check_collisions(&snake) {
                let canvas = app.get_canvas();
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                death = true;
            }

            if apple_check(&snake, &apple) {
                let last = snake.body.get(snake.body.len() - 1).unwrap();
                let dir = match snake.dir {
                    Direction::Up => Position::new(last.x, last.y + 1),
                    Direction::Down => Position::new(last.x, last.y - 1),
                    Direction::Left => Position::new(last.x - 1, last.y),
                    Direction::Right => Position::new(last.x + 1, last.y)
                };
                score += 1; apple.randomize(&snake); snake.body.push_back(dir);
            }

            app.present();
            fps = 0;
        }

        fps += 1;
        std::thread::sleep(Duration::from_millis(200))
    }
    Ok(())
}

fn restart(score: &mut i32, snake: &mut Snake, apple: &mut Apple, dead: &mut bool) {
    *score = 0; *dead = false;
    *snake = Snake::default();
    *apple = Apple::default();
}

pub mod snake {
    use std::collections::VecDeque;

    use sdl2::{render::WindowCanvas, pixels::Color, rect::Rect, ttf::*};

    const GAME_WIDTH: i32 = 30;
    const GAME_HEIGHT: i32 = 30;

    macro_rules! rect(
        ($x:expr, $y:expr, $w:expr, $h:expr) => (
            Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
        )
    );

    macro_rules! draw_text {
        ($font:expr, $style:expr, $color:expr, $canvas:expr, $text:expr, $rect:expr) => {
            $font.set_style($style);
            let f = $font.render($text).blended($color).map_err(|e| e.to_string()).unwrap();
            let tc = $canvas.texture_creator();
            $canvas.copy(
                &tc.create_texture_from_surface(&f).map_err(|e| e.to_string()).unwrap(),
                None,
                Some(
                    $rect
                )
            ).unwrap();
        };
        ($font:expr, $style:expr, $color:expr, $canvas:expr, $text:expr, $x:expr, $y:expr, $w:expr, $h:expr) => {
            $font.set_style($style);
            let f = $font.render($text).blended($color).map_err(|e| e.to_string()).unwrap();
            let tc = $canvas.texture_creator();
            $canvas.copy(
                &tc.create_texture_from_surface(&f).map_err(|e| e.to_string()).unwrap(),
                None,
                Some(
                    rect!($x, $y, $w, $h)
                )
            ).unwrap();
        };
    }

    pub fn display_death(canvas: &mut WindowCanvas, font: &mut Font, score: &i32) -> Result<(), String> {
        draw_text!(font, FontStyle::NORMAL, Color::RED, canvas, "YOU DIED", rect!(140, 200, 320, 200));
        draw_text!(font, FontStyle::NORMAL, Color::RED, canvas, &format!("Score: {}", score), rect!(140, 140, 100, 60));
        draw_text!(font, FontStyle::NORMAL, Color::WHITE, canvas, "Press space to restart", rect!(140, 400, 320, 80));
        // If the example text is too big for the screen, downscale it (and center irregardless)
        canvas.present();
        Ok(())
    }

    pub fn check_collisions(snake: &Snake) -> bool {
        let mut siter = snake.body.iter();

        let head = *siter.next().unwrap();

        if siter.any(|&x| head == x) {
            return true;
        }

        if (head.x > GAME_WIDTH || head.y > GAME_HEIGHT)
           || 
           (head.x < 0 || head.y < 0) 
        {
            return true;
        }

        false
    }

    pub fn apple_check(snake: &Snake, apple: &Apple) -> bool {
        let (ax, ay) = apple.get_position();
        for b in &snake.body {
            if b.x == ax && b.y == ay {
                return true;
            }
        }
        false
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    use crate::UNIT_SIZE;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Position {
        pub x: i32,
        pub y: i32
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    pub struct Apple {
        pos: Position
    }

    impl Default for Apple {
        fn default() -> Self {
            Self { pos: Position::new(24, 14) }
        }
    }
    
    fn get_rand() -> (i32, i32) {
        use rand::Rng;
        (
            rand::thread_rng().gen_range(0..GAME_WIDTH),
            rand::thread_rng().gen_range(0..GAME_HEIGHT)
        )
    }

    impl Apple {
        
        pub fn new(pos: Position) -> Self {
            Self { pos }
        }

        pub fn randomize(&mut self, snake: &Snake) {
            let (randx, randy) = get_rand();
            for b in &snake.body {
                if randx == b.x && randy == b.y {
                    self.randomize(snake);
                    return;
                }
            }
            self.pos.x = randx;
            self.pos.y = randy;
        }        

        pub fn get_position(&self) -> (i32, i32) {
            (self.pos.x, self.pos.y)
        }

        pub fn draw_apple(&self, can: &mut WindowCanvas) {
            can.set_draw_color(Color::RED);
            let (x, y) = self.get_position();
            can.fill_rect(
                Rect::new(
                    x * UNIT_SIZE as i32, 
                    y * UNIT_SIZE as i32, 
                    UNIT_SIZE as u32, 
                    UNIT_SIZE as u32
                )
            ).unwrap();
        }

    }

    pub struct Snake {
        pub body: VecDeque<Position>,
        pub dir: Direction
    }

    impl Default for Snake {
        fn default() -> Self {
            Snake::new(
                vec![
                    Position::new(4, 14),
                    Position::new(3, 14),
                    Position::new(2, 14)
                ].into()
            )
        }
    }

    impl Snake {

        pub fn new(
            body: VecDeque<Position>
        ) -> Self {
            Self {
                dir: Direction::Right,
                body,
            }
        }

        pub fn draw_snake(&self, canvas: &mut WindowCanvas) {
            canvas.set_draw_color(Color::CYAN);
    
            for pos in &self.body {
                canvas.fill_rect(
                    Rect::new(
                        pos.x*(UNIT_SIZE as i32), 
                        pos.y*(UNIT_SIZE as i32), 
                        UNIT_SIZE as u32, 
                        UNIT_SIZE as u32
                    )
                ).unwrap();
            }
        }

        /*const DIRS: [Direction; 4] = 
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up
        ];

        #[inline(always)]
        fn get_dir_val(&self) -> i32 {
            match self.dir {
                Direction::Up => 3,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Right => 0,
            }
        }

        #[inline]
        fn turn_left(&self) -> Direction {
            let mut i = self.get_dir_val();
            if (i + 1) <= 4 { i+=1; } else { i -= 4; }
            Self::DIRS[i as usize]
        }

        #[inline]
        fn turn_right(&self) -> Direction {
            let mut i = self.get_dir_val();
            if (i - 1) >= 0 { i-=1; } else { i += 4; }
            Self::DIRS[i as usize]
        }*/

        pub fn move_snake(&mut self, ai_enabled: &bool) {
            if *ai_enabled {
                /*ai::test(, )                

                self.dir = match out
                {
                    [0, 0, 1] => self.turn_left(),
                    [0, 1, 0] => self.turn_right(),
                    [1, 0, 0] => self.dir
                }*/
            }

            match self.dir {
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
            }
        }

        fn move_up(&mut self) {
            self.body.push_front(Position::new(self.body[0].x, self.body[0].y - 1));
            self.body.pop_back();
        }

        fn move_down(&mut self) {
            self.body.push_front(Position::new(self.body[0].x, self.body[0].y + 1));
            self.body.pop_back();
        }

        fn move_left(&mut self) {
            self.body.push_front(Position::new(self.body[0].x - 1, self.body[0].y));
            self.body.pop_back();
        }

        fn move_right(&mut self) {
            self.body.push_front(Position::new(self.body[0].x + 1, self.body[0].y));
            self.body.pop_back();
        }

    }

}
