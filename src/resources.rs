use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{Read, Write};

use ggez::audio::{SoundSource, Source};
use ggez::graphics::{FontData, Image, PxScale, Text, TextFragment};
use ggez::*;
use crate::types::{ButtonState, Color};

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
        let card_font = FontData::from_path(&ctx.fs, "/glacial/GlacialIndifference-Bold.ttf")?;
        let ui_font = FontData::from_path(&ctx.fs, "/glacial/GlacialIndifference-Bold.ttf")?;
        ctx.gfx.add_font("card_font", card_font);
        ctx.gfx.add_font("ui_font", ui_font);

        let mut numbers = Vec::new();
        for i in 1..10 {
            let nr = Text::new(
                TextFragment::new(i.to_string())
                    .font("card_font")
                    .scale(PxScale::from(32.0)),
            );
            //nr.set_filter(graphics::FilterMode::Nearest);
            numbers.push(nr);
        }

        let mut suite_icons = HashMap::new();
        suite_icons.insert(
            Color::Green,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/bamboo.png")?,
        );
        suite_icons.insert(
            Color::Red,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/coins.png")?,
        );
        suite_icons.insert(
            Color::White,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/characters.png")?,
        );

        let mut dragon_icons = HashMap::new();
        dragon_icons.insert(
            Color::Green,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/dragon_green.png")?,
        );
        dragon_icons.insert(
            Color::Red,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/dragon_red.png")?,
        );
        dragon_icons.insert(
            Color::White,
            Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/dragon_white.png")?,
        );

        let flower_icon = Image::from_path(&ctx.gfx, "/textures/solitaire/small_icons/flower.png")?;

        let mut suite_images = HashMap::new();
        let mut green = Vec::with_capacity(9);
        let mut white = Vec::with_capacity(9);
        let mut red = Vec::with_capacity(9);
        for i in 1..10 {
            let img = Image::from_path(
                &ctx.gfx,
                format!("/textures/solitaire/large_icons/bamboo_{}.png", i),
            )?;
            green.push(img);
            let img = Image::from_path(
                &ctx.gfx,
                format!("/textures/solitaire/large_icons/coins_{}.png", i),
            )?;
            red.push(img);
            let img = Image::from_path(
                &ctx.gfx,
                format!("/textures/solitaire/large_icons/char_{}.png", i),
            )?;
            white.push(img);
        }
        suite_images.insert(Color::Green, green);
        suite_images.insert(Color::Red, red);
        suite_images.insert(Color::White, white);

        let mut dragon_images = HashMap::new();
        dragon_images.insert(
            Color::Green,
            Image::from_path(&ctx.gfx, "/textures/solitaire/large_icons/dragon_green.png")?,
        );
        dragon_images.insert(
            Color::Red,
            Image::from_path(&ctx.gfx, "/textures/solitaire/large_icons/dragon_red.png")?,
        );
        dragon_images.insert(
            Color::White,
            Image::from_path(&ctx.gfx, "/textures/solitaire/large_icons/dragon_white.png")?,
        );

        let flower_image =
            Image::from_path(&ctx.gfx, "/textures/solitaire/large_icons/flower.png")?;

        let mut button_images = HashMap::new();
        button_images.insert(
            (Color::Green, ButtonState::Active),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_green_active.png")?,
        );
        button_images.insert(
            (Color::Green, ButtonState::Up),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_green_up.png")?,
        );
        button_images.insert(
            (Color::Green, ButtonState::Down),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_green_down.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Active),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_red_active.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Up),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_red_up.png")?,
        );
        button_images.insert(
            (Color::Red, ButtonState::Down),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_red_down.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Active),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_white_active.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Up),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_white_up.png")?,
        );
        button_images.insert(
            (Color::White, ButtonState::Down),
            Image::from_path(&ctx.gfx, "/textures/solitaire/button_white_down.png")?,
        );

        let r = Resources {
            wins: Resources::load_wins(ctx)?,
            table_image: Image::from_path(&ctx.gfx, "/textures/solitaire/table_large.png")?,
            card_front: Image::from_path(&ctx.gfx, "/textures/solitaire/card_front.png")?,
            card_back: Image::from_path(&ctx.gfx, "/textures/solitaire/card_back.png")?,
            numbers,
            suite_icons,
            dragon_icons,
            flower_icon,
            suite_images,
            dragon_images,
            flower_image,
            button_images,
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
                    .font("ui_font")
                    .scale(PxScale::from(56.0)),
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
        match ctx.fs.open("/wins.txt") {
            Ok(mut f) => {
                let mut string = String::new();
                f.read_to_string(&mut string)?;
                let n: u32 = string.parse().unwrap();
                Ok(n)
            }
            Err(GameError::ResourceNotFound(_, _)) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub fn store_wins(&self, ctx: &mut Context, n: u32) {
        let mut f = ctx.fs.create("/wins.txt").unwrap();
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
            Audio::None => {}
            Audio::Source(ref mut s) => s.play(),
        };
        Ok(())
    }
}
