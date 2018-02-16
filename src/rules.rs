
use cards::{Card, Color, Suite};
use cardstack::CardStack;

#[derive(Debug)]
pub enum StackRules {
    Dragging,
    Target,
    Dragon,
    Flower,
    Solitaire
}

impl StackRules {
    pub fn accept_drop(&self, top_card: Option<&Suite>, dropped: &Suite) -> bool {
        use self::Suite::*;
        match *self {
            StackRules::Dragging => panic!("Dragged stack can't accept a drop."),
            StackRules::Target => {
                match (top_card, dropped) {
                    (None, &Number(1, _)) => true,
                    (Some(&Number(i1, c1)), &Number(i2, c2)) => c1 == c2 && i2 == i1 + 1,
                    _ => false
                }
            }
            StackRules::Dragon => {
                match (top_card, dropped) {
                    (None, _) => true,
                    _ => false
                }
            },
            StackRules::Flower => {
                match (top_card, dropped) {
                    (None, &Flower) => true,
                    _ => false
                }
            }
            StackRules::Solitaire => {
                match (top_card, dropped) {
                    (None, _) => true,
                    (Some(&Number(i1, c1)), &Number(i2, c2)) => c1 != c2 && i2 + 1 == i1,
                    _ => false
                }
            },
        }
    }
}