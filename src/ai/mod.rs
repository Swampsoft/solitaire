use std::collections::{BinaryHeap, HashSet};

use rules;
use types::{Stack, StackRole, Suite};

pub enum AiResult {
    Unknown,
    Winable(i32),
    Lost
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AiState {
    stacks: Vec<Stack>
}

impl AiState {
    pub fn new(stacks: Vec<Stack>) -> AiState {
        AiState {
            stacks
        }
    }

    pub fn astar(&self, mut iterations: usize) -> AiResult {
        let mut queue = BinaryHeap::new();
        queue.push((0, self.clone()));

        let mut visited = HashSet::new();

        while let Some((depth, state)) = queue.pop() {
            if visited.contains(&state) {
                continue
            }

            if rules::check_victory(state.stacks.iter()) {
                return AiResult::Winable(depth)
            }

            iterations -= 1;
            if iterations == 0 {
                return AiResult::Unknown
            }

            visited.insert(state.clone());

            let moves = rules::calc_possible_moves(state.stacks.iter());
            for m in moves {
                let newstate = state.apply_move(m);
                queue.push((depth + 1, newstate));
            }
        }
        AiResult::Lost
    }

    fn apply_move(&self, m: rules::Move) -> AiState {
        let mut state = self.clone();
        match m {
            rules::Move::Button(_, t, s) => {
                state.stacks[s[0]].pop_card();
                state.stacks[s[1]].pop_card();
                state.stacks[s[2]].pop_card();
                state.stacks[s[3]].pop_card();
                state.stacks[t].push_card(Suite::FaceDown);
                state.stacks[t].push_card(Suite::FaceDown);
                state.stacks[t].push_card(Suite::FaceDown);
                state.stacks[t].push_card(Suite::FaceDown);
            }
            rules::Move::Cards(t, s, n) => {
                let i = state.stacks[s].len() - n;
                let tmp = state.stacks[s].split(i);
                state.stacks[t].extend(tmp);
            }
        }
        state
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for stack in &self.stacks {
            match stack.role {
                StackRole::Dragon => if let Some(Suite::FaceDown) = stack.top() {score += 100},
                StackRole::Target => if let Some(Suite::Number(n, _)) = stack.top() {score += 10 * n as i32}
                StackRole::Sorting => score += stack.score(),
                _ => {}
            }
        }
        score
    }
}

impl Ord for AiState {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for AiState {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Stack {
    fn score(&self) -> i32 {
        let a = self.cards.iter();
        let b = self.cards.iter().skip(1);
        let mut score = 0;
        for (l, u) in a.zip(b).rev() {
            if rules::is_valid_pair(*l, *u) {
                score += 1;
            } else {
                return score
            }
        }
        score
    }
}
