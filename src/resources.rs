
use std::collections::HashMap;

use ggez::graphics::{Font, Image, Text};
use ggez::*;

use button;
use cards;

pub struct Resources {
    pub table_image: Image,
    pub card_front: Image,
    pub numbers: Vec<Text>,
    pub suite_icons: HashMap<cards::Color, Image>,
    pub dragon_icons: HashMap<cards::Color, Image>,
    pub flower_icon: Image,
    pub suite_images: HashMap<cards::Color, Vec<Image>>,
    pub dragon_images: HashMap<cards::Color, Image>,
    pub flower_image: Image,
    pub button_images: HashMap<(cards::Color, button::State), Image>
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        //let card_font = Font::default_font()?;
        let card_font = Font::new(ctx, "/glacial/GlacialIndifference-Bold.ttf", 25)?;

        let mut numbers = Vec::new();
        for i in 1..10 {
            let mut nr = Text::new(ctx, &i.to_string(), &card_font)?;
            nr.set_filter(graphics::FilterMode::Nearest);
            numbers.push(nr);
        }

        let mut suite_icons = HashMap::new();
        suite_icons.insert(cards::Color::Green, Image::new(ctx, "/Content/textures/solitaire/small_icons/bamboo.png")?);
        suite_icons.insert(cards::Color::Red, Image::new(ctx, "/Content/textures/solitaire/small_icons/coins.png")?);
        suite_icons.insert(cards::Color::White, Image::new(ctx, "/Content/textures/solitaire/small_icons/characters.png")?);
        for img in suite_icons.values_mut() {
            img.set_filter(graphics::FilterMode::Nearest);
        }

        let mut dragon_icons = HashMap::new();
        dragon_icons.insert(cards::Color::Green, Image::new(ctx, "/Content/textures/solitaire/small_icons/dragon_green.png")?);
        dragon_icons.insert(cards::Color::Red, Image::new(ctx, "/Content/textures/solitaire/small_icons/dragon_red.png")?);
        dragon_icons.insert(cards::Color::White, Image::new(ctx, "/Content/textures/solitaire/small_icons/dragon_white.png")?);
        for img in dragon_icons.values_mut() {
            img.set_filter(graphics::FilterMode::Nearest);
        }

        let mut flower_icon = Image::new(ctx, "/Content/textures/solitaire/small_icons/flower.png")?;
        flower_icon.set_filter(graphics::FilterMode::Nearest);

        let mut suite_images = HashMap::new();
        let mut green = Vec::with_capacity(9);
        let mut white = Vec::with_capacity(9);
        let mut red = Vec::with_capacity(9);
        for i in 1..10 {
            let mut img = Image::new(ctx, format!("/Content/textures/solitaire/large_icons/bamboo_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            green.push(img);
            let mut img = Image::new(ctx, format!("/Content/textures/solitaire/large_icons/coins_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            red.push(img);
            let mut img = Image::new(ctx, format!("/Content/textures/solitaire/large_icons/char_{}.png", i))?;
            img.set_filter(graphics::FilterMode::Linear);
            white.push(img);
        }
        suite_images.insert(cards::Color::Green, green);
        suite_images.insert(cards::Color::Red, red);
        suite_images.insert(cards::Color::White, white);

        let mut dragon_images = HashMap::new();
        dragon_images.insert(cards::Color::Green, Image::new(ctx, "/Content/textures/solitaire/large_icons/dragon_green.png")?);
        dragon_images.insert(cards::Color::Red, Image::new(ctx, "/Content/textures/solitaire/large_icons/dragon_red.png")?);
        dragon_images.insert(cards::Color::White, Image::new(ctx, "/Content/textures/solitaire/large_icons/dragon_white.png")?);
        for img in dragon_images.values_mut() {
            img.set_filter(graphics::FilterMode::Linear);
        }

        let mut flower_image = Image::new(ctx, "/Content/textures/solitaire/large_icons/flower.png")?;
        flower_image.set_filter(graphics::FilterMode::Linear);

        let mut button_images = HashMap::new();
        button_images.insert((cards::Color::Green, button::State::Active), Image::new(ctx, "/Content/textures/solitaire/button_green_active.png")?);
        button_images.insert((cards::Color::Green, button::State::Up), Image::new(ctx, "/Content/textures/solitaire/button_green_up.png")?);
        button_images.insert((cards::Color::Green, button::State::Down), Image::new(ctx, "/Content/textures/solitaire/button_green_down.png")?);
        button_images.insert((cards::Color::Red, button::State::Active), Image::new(ctx, "/Content/textures/solitaire/button_red_active.png")?);
        button_images.insert((cards::Color::Red, button::State::Up), Image::new(ctx, "/Content/textures/solitaire/button_red_up.png")?);
        button_images.insert((cards::Color::Red, button::State::Down), Image::new(ctx, "/Content/textures/solitaire/button_red_down.png")?);
        button_images.insert((cards::Color::White, button::State::Active), Image::new(ctx, "/Content/textures/solitaire/button_white_active.png")?);
        button_images.insert((cards::Color::White, button::State::Up), Image::new(ctx, "/Content/textures/solitaire/button_white_up.png")?);
        button_images.insert((cards::Color::White, button::State::Down), Image::new(ctx, "/Content/textures/solitaire/button_white_down.png")?);

        let r = Resources {
            table_image: Image::new(ctx, "/Content/textures/solitaire/table_large.png")?,
            card_front: Image::new(ctx, "/Content/textures/solitaire/card_front.png")?,
            numbers,
            suite_icons,
            dragon_icons,
            flower_icon,
            suite_images,
            dragon_images,
            flower_image,
            button_images,
        };
        Ok(r)
    }
}