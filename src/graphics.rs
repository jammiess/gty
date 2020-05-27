/// All of the SFML interaction will be limited to this module.

use std::vec::Vec;

use sfml::graphics::*;
use sfml::system::{SfBox, Vector2i, Vector2f};
use sfml::window::{Style, Event};

const BACKGROUND_FILES: [&str; 13] = [
    "./res/bg01.bmp",
    "./res/bg02.bmp",
    "./res/bg03.bmp",
    "./res/bg04.bmp",
    "./res/bg05.bmp",
    "./res/bg06.bmp",
    "./res/bg07.bmp",
    "./res/bg08.bmp",
    "./res/bg09.bmp",
    "./res/bg10.bmp",
    "./res/bg11.bmp",
    "./res/bg12.bmp",
    "./res/bg13.bmp"
];
const PLAYER_FILE: &str = "./res/player.bmp";
const YOGURT_FILE: &str = "./res/yogurt.bmp";
const FONT_FILE: &str = "./res/data-latin.ttf";

pub struct Graphics
{
    /// Txtures will hold all of the data necessary to draw things in the game
    /// excluding text. If no error is thrown after a call to `load_textures`
    /// then this variable will look like `[player, yogurt, bg1, bg2, ... ]`.
    textures: Vec<SfBox<Texture>>,
    screen: RenderWindow,
    font: SfBox<Font>,
}

impl Graphics
{
    pub fn new() -> Self {
        let mut w = RenderWindow::new((640, 480),
                                      "Get the Yogurt",
                                      Style::CLOSE,
                                      &Default::default());
        w.set_framerate_limit(60);
        w.set_position(Vector2i::new(0, 0));
        w.set_active(true);
        w.set_key_repeat_enabled(false);
        Self {
            textures: Vec::new(),
            screen: w,
            font: Font::from_file(FONT_FILE).expect("Couldn't find font file."),
        }
    }

    pub fn load_textures(&mut self) -> Result<(),&'static str>
    {
        match Texture::from_file(PLAYER_FILE) {
            Some(player) => self.textures.push(player),
            None => return Err("Failed to load player texture")
        }
        match Texture::from_file(YOGURT_FILE) {
            Some(yogurt) => self.textures.push(yogurt),
            None => return Err("Failed to load yogurt")
        }
        for s in BACKGROUND_FILES.iter() {
            match Texture::from_file(s) {
                Some(texture) => {
                    self.textures.push(texture);
                },
                None => return Err("Failed to load all textures.")
            }
        }

        Ok(())
    }

    pub fn draw_title(&mut self, hs: u32, ls: u32)
    {
        let mut title = Text::new("Get the Yogurt!",
                             &self.font,
                             48);
        title.set_position(Vector2f { x: 180.0, y: 100.0 });
        
        let mut high_score = Text::new(&format!("High Score: {}", hs),
                                       &self.font,
                                       30);
        high_score.set_position(Vector2f { x: 251.0, y: 180.0 });

        let mut last_score = Text::new(&format!("Last Score: {}", ls),
                                       &self.font,
                                       30);
        last_score.set_position(Vector2f { x: 251.0, y: 230.0 });

        let mut start_message = Text::new("Press enter to play!",
                                          &self.font,
                                          30);
        start_message.set_position(Vector2f { x: 180.0, y: 280.0 });

        self.screen.draw(&Sprite::with_texture(&self.textures[14]));
        self.screen.draw_text(&title, Default::default());
        self.screen.draw_text(&high_score, Default::default());
        self.screen.draw_text(&last_score, Default::default());
        self.screen.draw_text(&start_message, Default::default());
    }

    pub fn draw_bg(&mut self, i: usize)
    {
        self.screen.draw(
            &Sprite::with_texture(&self.textures[i+2])
        );
    }

    pub fn display(&mut self)
    {
        self.screen.display();
    }

    pub fn num_bgs(&self) -> usize
    {
        self.textures.len() - 2
    }

    pub fn poll_event(&mut self) -> Option<Event>
    {
        self.screen.poll_event()
    }

    pub fn draw_player(&mut self, loc: Vector2f)
    {
        let mut sprite = Sprite::with_texture(&self.textures[0]);
        sprite.set_position(loc);
        self.screen.draw_sprite(&sprite, Default::default());
    }

    pub fn draw_score(&mut self, score: u32)
    {
        let mut text = Text::new(&format!("Score {}", score),
                             &self.font,
                             16);
        text.set_position(Vector2f { x: 10.0, y: 450.0 });
        self.screen.draw_text(&text, Default::default());
    }

    pub fn draw_yogurt(&mut self, loc: Vector2f)
    {
        let mut sprite = Sprite::with_texture(&self.textures[1]);
        sprite.set_position(loc);
        self.screen.draw_sprite(&sprite, Default::default());
    }
}
