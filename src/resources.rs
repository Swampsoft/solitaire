use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{Read, Write};

use ggez::audio::{SoundSource, Source};
use ggez::graphics::{Font, Image, Scale, Text, TextFragment};
use ggez::*;

use types::{ButtonState, Color};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sounds {
    None,
    //Pickup,
    Place,
    Sweep,
    Deal,
}

pub struct Resources {
    wins: u32,
    pub table_image: Image,
    pub card_front: Image,
    pub card_back: Image,
    pub numbers: Vec<Text>,
    pub suite_icons: HashMap<Color, Image>,
    pub dragon_icons: HashMap<Color, Image>,
    pub flower_icon: Image,
    pub suite_images: HashMap<Color, Vec<Image>>,
    pub dragon_images: HashMap<Color, Image>,
    pub flower_image: Image,
    pub button_images: HashMap<(Color, ButtonState), Image>,
    pub card_font: Font,
    pub ui_font: Font,
    pub text: HashMap<String, Text>,
    pub pickup_sound: Audio,
    pub place_sound: Audio,
    pub deal_sound: Audio,
    pub sweep_sound: Audio,
    pub music: Audio,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        //let card_font = Font::default_font()?;
        let card_font = Font::new(ctx, "/glacial/GlacialIndifference-Bold.ttf")?;
        let ui_font = Font::new(ctx, "/glacial/GlacialIndifference-Bold.ttf")?;

        let mut numbers = Vec::new();
        for i in 1..10 {
            let nr = Text::new(
                TextFragment::new(i.to_string())
                    .font(card_font)
                    .scale(Scale::uniform(32.0)),
            );
            //nr.set_filter(graphics::FilterMode::Nearest);
            numbers.push(nr);
        }

        let mut suite_icons = HashMap::new();
        suite_icons.insert(
            Color::Green,
            Image::new(ctx, "/textures/solitaire/small_icons/bamboo.png")?,
        );
        suite_icons.insert(
            Color::Red,
            Image::new(ctx, "/textures/solitaire/small_icons/coins.png")?,
        );
        suite_icons.insert(
            Color::White,
            Image::new(ctx, "/textures/solitaire/small_icons/characters.png")?,
        );
        for img in suite_icons.values_mut() {
            img.set_filter(graphics::FilterMode::Nearest);
        }

        let mut dragon_icons = HashMap::new();
        dragon_icons.insert(
            Color::Green,
            Image::new(ctx, "/textures/solitaire/small_icons/dragon_green.png")?,
        );
        dragon_icons.insert(
            Color::Red,
            Image::new(ctx, "/textures/solitaire/small_icons/dragon_red.png")?,
        );
        dragon_icons.insert(
            Color::White,
            Image::new(ctx, "/textures/solitaire/small_icons/dragon_white.png")?,
        );
        for img in dragon_icons.values_mut() {
            img.set_filter(graphics::FilterMode::Nearest);
        }

        let mut flower_icon = Image::new(ctx, "/textures/solitaire/small_icons/flower.png")?;
        flower_icon.set_filter(graphics::FilterMode::Nearest);

        let mut suite_images = HashMap::new();
        let mut green = Vec::with_capacity(9);
        let mut white = Vec::with_capacity(9);
        let mut red = Vec::with_capacity(9);
        for i in 1..10 {
            let mut img = Image::new(
                ctx,
                &format!("/textures/solitaire/large_icons/bamboo_{}.png", i),
            )?;
            img.set_filter(graphics::FilterMode::Linear);
            green.push(img);
            let mut img = Image::new(
                ctx,
                &format!("/textures/solitaire/large_icons/coins_{}.png", i),
            )?;
            img.set_filter(graphics::FilterMode::Linear);
            red.push(img);
            let mut img = Image::new(
                ctx,
                &format!("/textures/solitaire/large_icons/char_{}.png", i),
            )?;
            img.set_filter(graphics::FilterMode::Linear);
            white.push(img);
        }
        suite_images.insert(Color::Green, green);
        suite_images.insert(Color::Red, red);
        suite_images.insert(Color::White, white);

        let mut dragon_images = HashMap::new();
        dragon_images.insert(
            Color::Green,
            Image::new(ctx, "/textures/solitaire/large_icons/dragon_green.png")?,
        );
        dragon_images.insert(
            Color::Red,
            Image::new(ctx, "/textures/solitaire/large_icons/dragon_red.png")?,
        );
        dragon_images.insert(
            Color::White,
            Image::new(ctx, "/textures/solitaire/large_icons/dragon_white.png")?,
        );
        for img in dragon_images.values_mut() {
            img.set_filter(graphics::FilterMode::Linear);
        }

        let mut flower_image = Image::new(ctx, "/textures/solitaire/large_icons/flower.png")?;
        flower_image.set_filter(graphics::FilterMode::Linear);

        let mut button_images = HashMap::new();
        button_images.insert(
            (Color::Green, ButtonState::Active),
            Image::new(ctx, "/textures/solitaire/button_green_active.png")?,
        );
        button_images.insert(
            (Color::Green, ButtonState::Up),
            Image::new(ctx, "/textures/solitaire/button_green_up.png")?,
        );
        button_images.insert(
            (Color::Green, ButtonState::Down),
            Image::new(ctx, "/textures/solitaire/button_green_down.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Active),
            Image::new(ctx, "/textures/solitaire/button_red_active.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Up),
            Image::new(ctx, "/textures/solitaire/button_red_up.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Down),
            Image::new(ctx, "/textures/solitaire/button_red_down.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Active),
            Image::new(ctx, "/textures/solitaire/button_white_active.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Up),
            Image::new(ctx, "/textures/solitaire/button_white_up.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Down),
            Image::new(ctx, "/textures/solitaire/button_white_down.png")?,
        );

        let r = Resources {
            wins: Resources::load_wins(ctx)?,
            table_image: Image::new(ctx, "/textures/solitaire/table_large.png")?,
            card_front: Image::new(ctx, "/textures/solitaire/card_front.png")?,
            card_back: Image::new(ctx, "/textures/solitaire/card_back.png")?,
            numbers,
            suite_icons,
            dragon_icons,
            flower_icon,
            suite_images,
            dragon_images,
            flower_image,
            button_images,
            card_font,
            ui_font,
            text: HashMap::new(),
            pickup_sound: Audio::new(ctx, "/sounds/card_pickup.wav")?,
            place_sound: Audio::new(ctx, "/sounds/card_place.wav")?,
            deal_sound: Audio::new(ctx, "/sounds/card_deal.wav")?,
            sweep_sound: Audio::new(ctx, "/sounds/card_sweep.wav")?,
            music: Audio::new(ctx, "/music/Solitaire.ogg")?,
        };
        Ok(r)
    }

    pub fn get_text(&mut self, _ctx: &mut Context, s: &str) -> GameResult<&Text> {
        let text = match self.text.entry(s.to_owned()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Text::new(
                TextFragment::new(s)
                    .font(self.ui_font)
                    .scale(Scale::uniform(56.0)),
            )),
        };
        Ok(text)
    }

    pub fn play_sound(&mut self, sound: Sounds) {
        match sound {
            Sounds::None => return,
            //Sounds::Pickup => self.pickup_sound.play(),
            Sounds::Place => self.place_sound.play(),
            Sounds::Deal => self.deal_sound.play(),
            Sounds::Sweep => self.sweep_sound.play(),
        }
        .unwrap();
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }
    pub fn add_win(&mut self, ctx: &mut Context) {
        self.wins += 1;
        self.store_wins(ctx, self.wins);
    }

    fn load_wins(ctx: &mut Context) -> GameResult<u32> {
        match ggez::filesystem::open(ctx, "/wins.txt") {
            Ok(mut f) => {
                let mut string = String::new();
                f.read_to_string(&mut string).unwrap();
                let n: u32 = string.parse().unwrap();
                Ok(n)
            }
            Err(GameError::ResourceNotFound(_, _)) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub fn store_wins(&self, ctx: &mut Context, n: u32) {
        let mut f = ggez::filesystem::create(ctx, "/wins.txt").unwrap();
        f.write_all(format!("{}", n).as_bytes()).unwrap();
    }
}

pub enum Audio {
    Source(Source),
    None,
}

impl Audio {
    pub fn new(ctx: &mut Context, file: &str) -> GameResult<Audio> {
        match Source::new(ctx, file) {
            Ok(src) => Ok(Audio::Source(src)),
            Err(GameError::ResourceNotFound(_, _)) => Ok(Audio::None),
            Err(e) => Err(e),
        }
    }

    pub fn playing(&self) -> bool {
        match *self {
            Audio::None => true,
            Audio::Source(ref s) => s.playing(),
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        match *self {
            Audio::None => {}
            Audio::Source(ref mut s) => s.set_volume(vol),
        }
    }

    pub fn play(&mut self) -> GameResult<()> {
        match *self {
            Audio::None => Ok(()),
            Audio::Source(ref mut s) => s.play(),
        }
    }
}
