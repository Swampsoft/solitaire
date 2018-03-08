use std::collections::{HashSet, VecDeque};

use cs::GameState;
use rules;
use types::{Stack, Suite};

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

    // TODO: BFS would need a heuristic because the combinations explode quickly...
    pub fn dfs(&self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.clone());

        let mut visited = HashSet::new();

        while let Some(state) = queue.pop_back() {
            if visited.contains(&state) {
                continue
            }
            if rules::check_victory(state.stacks.iter()) {
                println!("OK");
                return
            }
            visited.insert(state.clone());
            let moves = rules::calc_possible_moves(state.stacks.iter());
            for m in moves {
                let newstate = state.apply_move(m);
                queue.push_back(newstate);
            }
        }
        println!(":-(");
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
}