use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Move {
    pub count: i32,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub struct Instructions {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

impl Instructions {
    fn format_stacks(stacks_str: &str) -> Vec<Vec<char>> {
        stacks_str
            .lines()
            .rev()
            .skip(1)
            .fold(Vec::new(), |stacks, line| {
                line.chars().chunks(4).into_iter().enumerate().fold(
                    stacks,
                    |mut stacks, (stack_idx, mut char_chunk)| {
                        if stacks.len() <= stack_idx {
                            stacks.push(Vec::new());
                        }

                        let stack = stacks.get_mut(stack_idx);
                        match (char_chunk.nth(1), stack) {
                            (Some(ch), Some(stack)) if ch != ' ' => stack.push(ch),
                            _ => (),
                        }

                        stacks
                    },
                )
            })
    }

    fn format_moves(moves_str: &str) -> Vec<Move> {
        moves_str
            .lines()
            .map(|line| {
                let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                let captures = re.captures(line).unwrap();
                Move {
                    count: captures.get(1).unwrap().as_str().parse().unwrap(),
                    from: captures.get(2).unwrap().as_str().parse().unwrap(),
                    to: captures.get(3).unwrap().as_str().parse().unwrap(),
                }
            })
            .collect()
    }
}

impl FromStr for Instructions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks_str, moves_str) = s.split("\n\n").collect_tuple().unwrap();
        let stacks: Vec<Vec<char>> = Self::format_stacks(stacks_str);
        let moves: Vec<Move> = Self::format_moves(moves_str);

        Ok(Instructions { stacks, moves })
    }
}
