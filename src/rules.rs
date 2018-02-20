
use button;
use cards::{Card, Color, Suite};
use table::Table;

#[derive(Debug)]
pub enum StackRules {
    Dragging,
    Target,
    Dragon,
    Flower,
    Solitaire
}

impl StackRules {
    pub fn accept_drag(&self, cards: &[Card]) -> bool {
        match *self {
            StackRules::Dragging => panic!("Dragged stack can't accept another drag."),
            StackRules::Target |
            StackRules::Flower => return false,
            StackRules::Dragon => {
                cards.iter().all(|c|c.is_faceup())
            },
            StackRules::Solitaire => {
                is_valid_stack(cards)
            },
        }
    }

    pub fn accept_drop(&self, top_card: Option<&Suite>, dropped: &Suite, n_cards: usize) -> bool {
        use self::Suite::*;
        match *self {
            StackRules::Dragging => panic!("Dragged stack can't accept a drop."),
            StackRules::Target if n_cards == 1 => {
                match (top_card, dropped) {
                    (None, &Number(1, _)) => true,
                    (Some(&Number(i1, c1)), &Number(i2, c2)) => c1 == c2 && i2 == i1 + 1,
                    _ => false
                }
            }
            StackRules::Dragon if n_cards == 1 => {
                match (top_card, dropped) {
                    (None, _) => true,
                    _ => false
                }
            },
            StackRules::Flower if n_cards == 1 => {
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
            _ => false
        }
    }
}

pub fn check_wincondition(table: &Table) -> bool {
    table.iter_solitaire_stacks()
    .map(|s|s.top_card())
    .all(|tc|tc.is_none())
}

pub fn is_valid_stack(cards: &[Card]) -> bool {
    use self::Suite::*;
    for (a, b) in cards.iter().zip(cards[1..].iter()) {
        match (a.suite(), b.suite()) {
            (&Number(ia, ca), &Number(ib, cb)) if ca != cb && ia == ib +1 => continue,
            _ => return false
        }
    }
    return true
}

pub fn global_rules(table: &mut Table) -> bool {
    use self::Suite::*;

    let mut n_green_dragons = 0;
    let mut n_red_dragons = 0;
    let mut n_white_dragons = 0;
    let mut auto_move = None;

    let mut dirty = false;

    'find_moves:
    for i in table.dragon_stacks().chain(table.solitaire_stacks()) {
        let top_suite = table.get_stack(i).top_suite();
        match top_suite {
            Some(&Flower) => {
                auto_move = Some((i, table.flower_stack()));
                break 'find_moves;
            },
            Some(&Dragon(Color::Green)) => n_green_dragons += 1,
            Some(&Dragon(Color::Red)) => n_red_dragons += 1,
            Some(&Dragon(Color::White)) => n_white_dragons += 1,
            _ => {}
        }

        let i_min = table.iter_target_stacks().map(|stack| {
            match stack.top_suite() {
                Some(&Number(i, _)) => i,
                None => 0,
                _ => panic!("Invalid card on target stack")
            }
        }).min().unwrap_or(0);

        for t in table.target_stacks() {
            let target_suite = table.get_stack(t).top_suite();
            match (top_suite, target_suite) {
                (Some(&Number(1, _)), None) => {},
                (Some(&Number(i2, c2)), Some(&Number(i1, c1))) if c1 == c2 && i2 == i1 + 1 && i2 == i_min + 1=> {},
                _ => continue
            }
            auto_move = Some((i, t));
            break 'find_moves;
        }
    }

    for b in 0..table.n_buttons() {
        if let button::ButtonState::Down = table.get_button(b).state() {
            continue
        }
        let color = table.get_button(b).color();
        if table.find_dragon_target(color).is_none() {
            continue
        }
        match color {
            Color::Green if n_green_dragons == 4 => table.set_button(b, button::ButtonState::Active),
            Color::Red if n_red_dragons == 4 => table.set_button(b, button::ButtonState::Active),
            Color::White if n_white_dragons == 4 => table.set_button(b, button::ButtonState::Active),
            _ => table.set_button(b, button::ButtonState::Up)
        }
    }

    if let Some((s, t)) = auto_move {
        table.animate_move(s, t);
        dirty = false;
    }

    dirty
}
