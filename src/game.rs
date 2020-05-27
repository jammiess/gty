use sfml::window::{Event, Key};
use sfml::system::Vector2f;

use random::{Xorshift128Plus, Source};
use std::time::{SystemTime, Duration};

use crate::graphics;

const PLAYER_SPEED: f32 = 5.0;

pub struct Game
{
    graphics: graphics::Graphics,
    player: Vector2f,
    yogurt: Vector2f,
    high_score: u32,
    last_score: u32,
    gen: random::Xorshift128Plus,
}

impl Game
{
    pub fn new(s1: u64, s2: u64) -> Self
    {
        Self {
            graphics: graphics::Graphics::new(),
            player: Vector2f { x: 320.0, y: 240.0 },
            yogurt: Vector2f { x: 0.0, y: 0.0 },
            high_score: 0,
            last_score: 0,
            gen: Xorshift128Plus::new([s1, s2]),
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str>
    {
        self.graphics.load_textures()
    }

    pub fn game_loop(&mut self)
    {
        let time_allowed = Duration::from_secs(30);
        'event_loop: loop {
            let mut frame: u32 = 0;
            let mut curr_bg = 0;
            let num_bgs = self.graphics.num_bgs();
            let mut curr_score = 0;
            let mut movement: [bool; 4] = [false; 4];
            self.yogurt = new_position(&mut self.gen);

            self.graphics.draw_title(self.high_score, self.last_score);
            self.graphics.display();
            'start_loop: loop {
                if let Some(e) = self.graphics.poll_event() {
                    match e {
                        Event::Closed => break 'event_loop,
                        Event::KeyPressed { code: Key::Return, .. } => break 'start_loop,
                        Event::KeyPressed { code: Key::Q, .. } => break 'event_loop,
                        _ => {}
                    }
                }
            }

            let start_time = SystemTime::now();
            'game_loop: loop {
                frame += 1;
                while let Some(e) = self.graphics.poll_event()
                {
                    match e {
                        Event::Closed => break 'event_loop,
                        Event::KeyPressed { code: Key::Q, .. } => break 'game_loop,
                        Event::KeyPressed { code: Key::Left, .. } => movement[0] = true,
                        Event::KeyReleased { code: Key::Left, .. } => movement[0] = false,
                        Event::KeyPressed { code: Key::Down, .. } => movement[1] = true,
                        Event::KeyReleased { code: Key::Down, .. } => movement[1] = false,
                        Event::KeyPressed { code: Key::Up, .. } => movement[2] = true,
                        Event::KeyReleased { code: Key::Up, .. } => movement[2] = false,
                        Event::KeyPressed { code: Key::Right, .. } => movement[3] = true,
                        Event::KeyReleased { code: Key::Right, .. } => movement[3] = false,
                        _ => {}
                    }
                }

                if movement[0] {    // left
                    self.player.x = (self.player.x - PLAYER_SPEED).max(0.0);
                }
                if movement[1] {    // down
                    self.player.y = (self.player.y + PLAYER_SPEED).min(480.0 - 32.0);
                }
                if movement[2] {    // up
                    self.player.y = (self.player.y - PLAYER_SPEED).max(0.0);
                }
                if movement[3] {    // right
                    self.player.x = (self.player.x + PLAYER_SPEED).min(640.0 - 32.0);
                }

                if eaten(self.player, self.yogurt) {
                    curr_score += 1;
                    self.yogurt = new_position(&mut self.gen);
                }

                if frame % 15 == 0 {
                    frame = 0;
                    curr_bg = (curr_bg + 1) % num_bgs;
                }

                self.graphics.draw_bg(curr_bg);
                self.graphics.draw_player(self.player);
                self.graphics.draw_yogurt(self.yogurt);
                self.graphics.draw_score(curr_score);
                self.graphics.display();

                if start_time.elapsed().expect("Timer Failed") > time_allowed {
                    self.last_score = curr_score;
                    self.high_score = curr_score.max(self.high_score);
                    break 'game_loop;
                }
            }
        }
    }
}

pub fn eaten(p: Vector2f, y: Vector2f) -> bool
{
    let dist = (p.x - y.x).powi(2) + (p.y - y.y).powi(2);
    return dist < 900.0;
}

pub fn new_position(generator: &mut Xorshift128Plus) -> Vector2f
{
    Vector2f {
        x: (generator.read_f64() * (640.0 - 32.0)) as f32,
        y: (generator.read_f64() * (480.0 - 32.0)) as f32
    }
}
