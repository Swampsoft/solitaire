
use ggez::graphics::Image;
use ggez::*;

pub struct Resources {
    pub table_image: Image,
    pub card_front: Image,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        let r = Resources {
            table_image: Image::new(ctx, "/table_large.png")?,
            card_front: Image::new(ctx, "/card_front.png")?,
        };
        Ok(r)
    }
}