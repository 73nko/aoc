use std::collections::HashSet;

type Moves = Vec<(i32, i32, usize)>;

fn part1(moves: &Moves) -> usize {
    simulate_rope(moves, 1)
}
fn part2(moves: &Moves) -> usize {
    simulate_rope(moves, 9)
}

fn prepare_input(input: &str) -> Moves {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            let (dx, dy) = match a.as_bytes()[0] as char {
                'U' => (0, 1),
                'D' => (0, -1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => unreachable!(),
            };
            (dx, dy, b.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn simulate_rope(moves: &[(i32, i32, usize)], followers: usize) -> usize {
    let mut rope = vec![(0i32, 0i32); followers + 1];
    let mut visited = HashSet::with_capacity(10000);
    for &(dx, dy, len) in moves {
        for _ in 0..len {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[followers]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2"#;
    #[test]
    fn test_part1() {
        let moves = prepare_input(INPUT);
        assert_eq!(part1(&moves), 13);
    }
    #[test]
    fn test_part2() {
        let moves = prepare_input(INPUT);
        assert_eq!(part2(&moves), 1);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = prepare_input(input);
    println!("PART 1: {:?}", part1(&moves));
    println!("PART 2: {:?}", part2(&moves));
}
