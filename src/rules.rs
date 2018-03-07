
use types::*;

pub fn is_valid_pair(lower: Suite, upper: Suite) -> bool {
    use self::Suite::*;
    match (lower, upper) {
        (Number(ln, lc), Number(un, uc)) => lc != uc && ln == un + 1,
        _ => false
    }
}

pub fn is_valid_sequence<'a, T: Iterator<Item=&'a Suite>>(cards: T) -> bool {
    let mut iter = cards.into_iter();
    let mut lower = match iter.next() {
        None => return true,  // an empty sequence is a valid sequence
        Some(&l) => l,
    };
    for &upper in iter {
        if !is_valid_pair(lower, upper) {
            return false
        }
        lower = upper;
    }
    true
}

pub fn is_valid_drag(stack: &Stack, idx: usize) -> bool {
    match (stack.role, stack.top()) {
        (_, None) => false,
        (StackRole::Flower, _) => false,
        (StackRole::Target, _) => false,
        (StackRole::Dragon, Some(card)) => card != Suite::FaceDown,
        (StackRole::Sorting, _) => is_valid_sequence(stack.cards[idx..].iter()),
        (StackRole::Generic, _) |
        (StackRole::Animation, _) => panic!("Attempt to drag from invalid stack")
    }
}

pub fn is_valid_drop(target: &Stack, source: &Stack) -> bool {
    let base_card = source.cards[0];
    let n_cards = source.len();

    is_valid_move(target, base_card, n_cards)
}

pub fn is_valid_move(target: &Stack, base_card: Suite, n_cards: usize) -> bool {
    use self::Suite::*;

    let top_card = target.top();

    match (target.role, top_card, base_card, n_cards) {
        (StackRole::Dragon, None, _, 1) => true,
        (StackRole::Flower, None, Flower, 1) => true,
        (StackRole::Target, None, Number(1, _), 1) => true,
        (StackRole::Target, Some(Number(ln, lc)), Number(un, uc), 1) => lc == uc && ln + 1 == un,
        (StackRole::Sorting, None, _, _) => true,
        (StackRole::Sorting, Some(l), u, _) => is_valid_pair(l, u),
        (StackRole::Generic, _, _, _) |
        (StackRole::Animation, _, _, _) => panic!("Attempt to drop on invalid stack"),
        _ => false,
    }
}

pub fn check_button<'a, I: Iterator<Item=&'a Stack> + Clone>(color: Color, stacks: I)  -> Option<(usize, [usize; 4])> {
    let target = stacks.clone().enumerate()
        .filter(|&(_, ref stack)| stack.role == StackRole::Dragon)
        .filter(|&(_, ref stack)| match stack.top() {
            Some(Suite::Dragon(col)) => col == color,       // only dragons of right color
            None => true,                                   // or empty stack
            _ => false,
        })
        .map(|(i, _)| i)
        .next();

    let target = match target {
        Some(t) => t,
        None => return None,
    };

    let source_it = stacks.enumerate()
        .filter(|&(_, ref stack)| match stack.top() {                 // only dragons of right color
            Some(Suite::Dragon(col)) => color == col,
            _ => false,
        })
        .map(|(i, _)| i);

    let mut sources = [0; 4];

    let mut n = 0;
    for (i, s) in source_it.enumerate() {
        sources[i] = s;
        n = i + 1;
    }

    if n == 4 {
        Some((target, sources))
    } else {
        None
    }
}

pub fn get_automove<'a, I: Iterator<Item=&'a Stack> + Clone>(stacks: I) -> Option<(usize, usize)> {
    use self::Suite::*;

    let lowest_nr = stacks.clone().filter_map(|s| match (s.role, s.top()) {
        (StackRole::Target, None) => Some(0),
        (StackRole::Target, Some(Number(n, _))) => Some(n),
        _ => None
    }).min().unwrap();

    for (i, t_stack) in stacks.clone().enumerate() {
        match t_stack.role {
            StackRole::Target | StackRole::Flower => {}
            _ => continue
        }

        for (j, s_stack) in stacks.clone().enumerate() {
            match s_stack.role {
                StackRole::Dragon | StackRole::Sorting => {}
                _ => continue
            }

            match (s_stack.top(), t_stack.top()) {
                //(Flower, None, StackRole::Flower) =>
                (Some(Number(n, c)), _) => if n <= lowest_nr + 1 && is_valid_move(t_stack, Number(n, c), 1) {
                    return Some((i, j))
                }
                (Some(card), _) => if is_valid_move(t_stack, card, 1) {
                    return Some((i, j))
                }
                _ => continue
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_pairs() {
        use self::Suite::*;
        use self::Color::*;

        for i in 1..9 {
            assert!(is_valid_pair(Number(i + 1, Green), Number(i, Red)));
            assert!(is_valid_pair(Number(i + 1, Green), Number(i, White)));
            assert!(is_valid_pair(Number(i + 1, White), Number(i, Red)));
            assert!(is_valid_pair(Number(i + 1, Red), Number(i, Green)));
            assert!(is_valid_pair(Number(i + 1, White), Number(i, Green)));
            assert!(is_valid_pair(Number(i + 1, Red), Number(i, White)));
            assert!(!is_valid_pair(Number(i + 1, Red), Number(i, Red)));
            assert!(!is_valid_pair(Number(i + 1, Green), Number(i, Green)));
            assert!(!is_valid_pair(Number(i + 1, White), Number(i, White)));
        }

        for &c1 in &[Red, Green, White] {
            for &c2 in &[Red, Green, White] {
                for i in 1..10 {
                    for j in 1..10 {
                        if i == j + 1 {
                            continue
                        }
                        assert!(!is_valid_pair(Number(i, c1), Number(j, c2)));
                    }
                    assert!(!is_valid_pair(Number(i, c1), Dragon(c2)));
                    assert!(!is_valid_pair(Number(i, c1), Flower));
                    assert!(!is_valid_pair(Dragon(c1), Number(i, c2)));
                    assert!(!is_valid_pair(Flower, Number(i, c2)));
                }
                assert!(!is_valid_pair(Dragon(c1), Dragon(c2)));
            }
            assert!(!is_valid_pair(Dragon(c1), Flower));
            assert!(!is_valid_pair(Flower, Dragon(c1)));
        }
        assert!(!is_valid_pair(Flower, Flower));
    }

    #[test]
    fn valid_sequences() {
        use self::Suite::*;
        use self::Color::*;
        use std::iter;

        let valid_vec = vec![Number(5, White), Number(4, Red), Number(3, White)];
        let valid_slice = &[Number(9, Red), Number(8, Green), Number(7, White)];

        let valid_iter = iter::once(&Number(3, Green))
            .chain(iter::once(&Number(2, Red)))
            .chain(iter::once(&Number(1, White)));

        assert!(is_valid_sequence(valid_vec.iter()));
        assert!(is_valid_sequence(valid_vec.iter()));
        assert!(is_valid_sequence(Vec::new().iter()));  // empty sequence

        assert!(is_valid_sequence(valid_slice.iter()));
        assert!(is_valid_sequence(valid_slice.iter()));
        assert!(is_valid_sequence([].iter()));  // empty sequence
        assert!(is_valid_sequence([Number(42, Red)].iter()));  // single item

        assert!(is_valid_sequence(valid_iter));
        assert!(is_valid_sequence(iter::empty()));  // empty sequence
        assert!(is_valid_sequence(iter::once(&Number(6, White))));  // single item

        assert!(!is_valid_sequence([Number(3, Red), Number(2, Red), Number(1, Red)].iter()));
        assert!(!is_valid_sequence([Number(3, Red), Number(2, Green), Dragon(White)].iter()));
        assert!(!is_valid_sequence([Number(3, Red), Number(2, Green), Flower].iter()));
    }

}