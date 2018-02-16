
use button;
use cards::{Card, Color, Suite};
use mainstate::MainState;

#[derive(Debug)]
pub enum StackRules {
    Dragging,
    Target,
    Dragon,
    Flower,
    Solitaire
}

impl StackRules {
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

pub fn valid_stack(cards: &[Card]) -> bool {
    use self::Suite::*;
    for (a, b) in cards.iter().zip(cards[1..].iter()) {
        match (a.suite(), b.suite()) {
            (&Number(ia, ca), &Number(ib, cb)) if ca != cb && ia == ib +1 => continue,
            _ => return false
        }
    }
    return true
}

pub fn global_rules(state: &mut MainState) {
    use self::Suite::*;

    let mut n_green_dragons = 0;
    let mut n_red_dragons = 0;
    let mut n_white_dragons = 0;
    let mut auto_move = Vec::new();

    for i in state.dragon_and_solitaire_stacks() {
        let top_suite = state.stacks[i].top_suite();
        match top_suite {
            Some(&Flower) => auto_move.push((i, state.flower_stack)),
            Some(&Dragon(Color::Green)) => n_green_dragons += 1,
            Some(&Dragon(Color::Red)) => n_red_dragons += 1,
            Some(&Dragon(Color::White)) => n_white_dragons += 1,
            _ => {}
        }

        let i_min = state.target_stacks.iter().map(|s| {
            match state.stacks[*s].top_suite() {
                Some(&Number(i, _)) => i,
                None => 0,
                _ => panic!("Invalid state of target stack")
            }
        }).min().unwrap_or(0);

        for t in &state.target_stacks {
            let target_suite = state.stacks[*t].top_suite();
            match (top_suite, target_suite) {
                (Some(&Number(1, _)), None) => {},
                (Some(&Number(i2, c2)), Some(&Number(i1, c1))) if c1 == c2 && i2 == i1 + 1 && i2 == i_min + 1=> {},
                _ => continue
            }
            auto_move.push((i, *t));
            break
        }
    }

    for (s, t) in auto_move {
        let card = state.stacks[s].pop().unwrap();
        state.stacks[t].push_card(card);
        state.set_dirty();
    }

    for b in 0..state.buttons.len() {
        if let button::State::Down = state.buttons[b].state() {
            continue
        }
        let color = state.buttons[b].color();
        if state.find_dragon_target(color).is_none() {
            continue
        }
        match color {
            Color::Green if n_green_dragons == 4 => state.buttons[b].set_state(button::State::Active),
            Color::Red if n_red_dragons == 4 => state.buttons[b].set_state(button::State::Active),
            Color::White if n_white_dragons == 4 => state.buttons[b].set_state(button::State::Active),
            _ => state.buttons[b].set_state(button::State::Up)
        }
    }
}
