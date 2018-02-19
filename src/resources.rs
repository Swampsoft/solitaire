
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use ggez::graphics::{Font, Image, Text};
use ggez::audio::Source;
use ggez::*;

use button;
use cards;

#[derive(Copy, Clone)]
pub enum Sounds {
    None,
    Pickup,
    Place,
    Sweep,
    Deal,
}

pub struct Resources {
    pub table_image: Image,
    pub card_front: Image,
    pub card_back: Image,
    pub numbers: Vec<Text>,
    pub suite_icons: HashMap<cards::Color, Image>,
    pub dragon_icons: HashMap<cards::Color, Image>,
    pub flower_icon: Image,
    pub suite_images: HashMap<cards::Color, Vec<Image>>,
    pub dragon_images: HashMap<cards::Color, Image>,
    pub flower_image: Image,
    pub button_images: HashMap<(cards::Color, button::ButtonState), Image>,
    pub card_font: Font,
    pub ui_font: Font,
    pub text: HashMap<String, Text>,
    pub pickup_sound: Source,
    pub place_sound: Source,
    pub deal_sound: Source,
    pub sweep_sound: Source,
    pub music: Source,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        //let card_font = Font::default_font()?;
        let card_font = Font::new(ctx, "/glacial/GlacialIndifference-Bold.ttf", 25)?;
        let ui_font = Font::new(ctx, "/glacial/GlacialIndifference-Bold.ttf", 42)?;

        let mut numbers = Vec::new();
        for i in 1..10 {
            let mut nr = Text::new(ctx, &i.to_string(), &card_font)?;
            nr.set_filter(graphics::FilterMode::Nearest);
            numbers.push(nr);
        }

        let mut suite_icons = HashMap::new();
        suite_icons.insert(cards::Color::Green, Image::new(ctx, "/textures/solitaire/small_icons/bamboo.png")?);
        suite_icons.insert(cards::Color::Red, Image::new(ctx, "/textures/solitaire/small_icons/coins.png")?);
        suite_icons.insert(cards::Color::White, Image::new(ctx, "/textures/solitaire/small_icons/characters.png")?);
        for img in suite_icons.values_mut() {
            img.set_filter(graphics::FilterMode::Nearest);
        }

        let mut dragon_icons = HashMap::new();
        dragon_icons.insert(cards::Color::Green, Image::new(ctx, "/textures/solitaire/small_icons/dragon_green.png")?);
        dragon_icons.insert(cards::Color::Red, Image::new(ctx, "/textures/solitaire/small_icons/dragon_red.png")?);
        dragon_icons.insert(cards::Color::White, Image::new(ctx, "/textures/solitaire/small_icons/dragon_white.png")?);
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
            let mut img = Image::new(ctx, format!("/textures/solitaire/large_icons/bamboo_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            green.push(img);
            let mut img = Image::new(ctx, format!("/textures/solitaire/large_icons/coins_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            red.push(img);
            let mut img = Image::new(ctx, format!("/textures/solitaire/large_icons/char_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            white.push(img);
        }
        suite_images.insert(cards::Color::Green, green);
        suite_images.insert(cards::Color::Red, red);
        suite_images.insert(cards::Color::White, white);

        let mut dragon_images = HashMap::new();
        dragon_images.insert(cards::Color::Green, Image::new(ctx, "/textures/solitaire/large_icons/dragon_green.png")?);
        dragon_images.insert(cards::Color::Red, Image::new(ctx, "/textures/solitaire/large_icons/dragon_red.png")?);
        dragon_images.insert(cards::Color::White, Image::new(ctx, "/textures/solitaire/large_icons/dragon_white.png")?);
        for img in dragon_images.values_mut() {
            img.set_filter(graphics::FilterMode::Linear);
        }

        let mut flower_image = Image::new(ctx, "/textures/solitaire/large_icons/flower.png")?;
        flower_image.set_filter(graphics::FilterMode::Linear);

        let mut button_images = HashMap::new();
        button_images.insert((cards::Color::Green, button::ButtonState::Active), Image::new(ctx, "/textures/solitaire/button_green_active.png")?);
        button_images.insert((cards::Color::Green, button::ButtonState::Up), Image::new(ctx, "/textures/solitaire/button_green_up.png")?);
        button_images.insert((cards::Color::Green, button::ButtonState::Down), Image::new(ctx, "/textures/solitaire/button_green_down.png")?);
        button_images.insert((cards::Color::Red, button::ButtonState::Active), Image::new(ctx, "/textures/solitaire/button_red_active.png")?);
        button_images.insert((cards::Color::Red, button::ButtonState::Up), Image::new(ctx, "/textures/solitaire/button_red_up.png")?);
        button_images.insert((cards::Color::Red, button::ButtonState::Down), Image::new(ctx, "/textures/solitaire/button_red_down.png")?);
        button_images.insert((cards::Color::White, button::ButtonState::Active), Image::new(ctx, "/textures/solitaire/button_white_active.png")?);
        button_images.insert((cards::Color::White, button::ButtonState::Up), Image::new(ctx, "/textures/solitaire/button_white_up.png")?);
        button_images.insert((cards::Color::White, button::ButtonState::Down), Image::new(ctx, "/textures/solitaire/button_white_down.png")?);

        let r = Resources {
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
            pickup_sound: Source::new(ctx, "/sounds/card_pickup.wav")?,
            place_sound: Source::new(ctx, "/sounds/card_place.wav")?,
            deal_sound: Source::new(ctx, "/sounds/card_deal.wav")?,
            sweep_sound: Source::new(ctx, "/sounds/card_sweep.wav")?,
            music: Source::new(ctx, "/music/Solitaire.ogg")?,
        };
        Ok(r)
    }

    pub fn get_text(&mut self, ctx: &mut Context, s: &str) -> GameResult<&Text> {
        let text = match self.text.entry(s.to_owned()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Text::new(ctx, s, &self.ui_font)?)
        };
        Ok(text)
    }

    pub fn play_sound(&self, sound: Sounds) {
        match sound {
            Sounds::None => return,
            Sounds::Pickup => self.pickup_sound.play(),
            Sounds::Place => self.place_sound.play(),
            Sounds::Deal => self.deal_sound.play(),
            Sounds::Sweep => self.sweep_sound.play(),
        }.unwrap();
    }
}
