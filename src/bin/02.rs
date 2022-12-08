#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
use Move::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum State {
    Win,
    Loss,
    Draw,
}
use State::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Match {
    my_move: Option<Move>,
    their_move: Move,
}

lazy_static! {
    static ref MOVE_SCORES: HashMap<Move, u32> = {
        let mut m = HashMap::new();
        m.insert(Rock, 1);
        m.insert(Paper, 2);
        m.insert(Scissors, 3);
        m
    };
    static ref STATE_SCORES: HashMap<State, u32> = {
        let mut m = HashMap::new();
        m.insert(Loss, 0);
        m.insert(Win, 6);
        m.insert(Draw, 3);
        m
    };
    static ref CODE_TO_MOVE: HashMap<char, Move> = {
        let mut m = HashMap::new();
        m.insert('A', Rock);
        m.insert('B', Paper);
        m.insert('C', Scissors);
        m.insert('X', Rock);
        m.insert('Y', Paper);
        m.insert('Z', Scissors);
        m
    };
    static ref WINS: HashSet<(Move, Move)> = {
        let mut m = HashSet::new();
        m.insert((Scissors, Rock));
        m.insert((Rock, Paper));
        m.insert((Paper, Scissors));
        m
    };
}

impl Match {
    pub fn new(my_move: Option<Move>, their_move: Move) -> Self {
        Self {
            my_move,
            their_move,
        }
    }

    pub fn state(&self) -> Option<State> {
        if self.my_move.is_none() {
            None
        } else if self.my_move.unwrap() == self.their_move {
            Some(Draw)
        } else if WINS.contains(&(self.their_move, self.my_move.unwrap())) {
            Some(Win)
        } else {
            Some(Loss)
        }
    }

    pub fn update_to_state(&mut self, desired_state: State) -> Self {
        self.my_move = if desired_state == Draw {
            Some(self.their_move)
        } else if desired_state == Win {
            match self.their_move {
                Rock => Some(Paper),
                Paper => Some(Scissors),
                Scissors => Some(Rock),
            }
        } else {
            match self.their_move {
                Rock => Some(Scissors),
                Scissors => Some(Paper),
                Paper => Some(Rock),
            }
        };
        *self
    }
    pub fn get_score(&self) -> Option<u32> {
        self.my_move.map(|my_move| {
            let move_score = MOVE_SCORES.get(&my_move).unwrap();
            let result = self.state().unwrap();
            let state_score = STATE_SCORES.get(&result).unwrap();
            *move_score + *state_score
        })
    }
}

fn parse_match(input: &str) -> (&Move, &Move) {
    input
        .chars()
        .filter_map(|c| CODE_TO_MOVE.get(&c))
        .collect_tuple()
        .unwrap()
}

fn parse_match_2(input: &str) -> Match {
    let (move_char, state_char): (char, char) = input
        .chars()
        .filter(|c| *c != ' ')
        .take(2)
        .collect_tuple()
        .unwrap();
    let their_move: &Move = CODE_TO_MOVE.get(&move_char).unwrap();
    let desired_state: &State = match state_char {
        'X' => &Loss,
        'Y' => &Draw,
        _ => &Win,
    };
    Match::new(None, *their_move).update_to_state(*desired_state)
}

fn match_state((a, b): (&Move, &Move)) -> State {
    if a == b {
        Draw
    } else if WINS.contains(&(*a, *b)) {
        Win
    } else {
        Loss
    }
}

fn calculate_match_score((a, b): (&Move, &Move)) -> u32 {
    let move_score = MOVE_SCORES.get(b).unwrap();
    let result = match_state((a, b));
    let state_score = STATE_SCORES.get(&result).unwrap();
    let total_score = *move_score + *state_score;
    dbg!(
        a,
        b,
        result,
        move_score,
        state_score,
        total_score,
        "-------"
    );

    total_score
}

pub fn part_one(input: &str) -> Option<u32> {
    let rps_matches: Vec<(&Move, &Move)> = input.lines().map(|rps| parse_match(rps)).collect();
    dbg!(&rps_matches);
    let points: u32 = rps_matches
        .iter()
        .map(|(a, b)| calculate_match_score((a, b)))
        .sum();
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let items: u32 = input
        .lines()
        .map(parse_match_2)
        .map(|m| m.get_score().unwrap_or(0))
        .sum();
    dbg!(items);

    Some(items)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
