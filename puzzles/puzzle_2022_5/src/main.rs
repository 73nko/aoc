mod instructions;

use instructions::{Instructions, Move};

fn part1(input: &str) -> String {
    let Instructions { mut stacks, moves } = input.parse::<Instructions>().unwrap();
    crate_mover_9000(&mut stacks, &moves);
    stacks.iter().filter_map(|s| s.last()).collect()
}
fn part2(input: &str) -> String {
    let Instructions { mut stacks, moves } = input.parse::<Instructions>().unwrap();
    crate_mover_9001(&mut stacks, &moves);
    stacks.iter().filter_map(|s| s.last()).collect()
}

fn crate_mover_9000(stacks: &mut [Vec<char>], moves: &[Move]) {
    moves.iter().for_each(|Move { count, from, to }| {
        for _ in 0..*count {
            let from_stack = stacks.get_mut(from - 1).unwrap();
            let val = from_stack.pop().unwrap();
            let to_stack = stacks.get_mut(to - 1).unwrap();
            to_stack.push(val);
        }
    });
}

fn crate_mover_9001(stacks: &mut [Vec<char>], moves: &[Move]) {
    moves.iter().for_each(|Move { count, from, to }| {
        let from_stack = stacks.get_mut(from - 1).unwrap();
        let split_point = from_stack.len() - *count as usize;
        let mut moving = from_stack.drain(split_point..).collect();
        let to_stack = stacks.get_mut(to - 1).unwrap();

        to_stack.append(&mut moving);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "CMZ");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "MCD");
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("PART 1: {:?}", part1(input));
    println!("PART 2: {:?}", part2(input));
}
