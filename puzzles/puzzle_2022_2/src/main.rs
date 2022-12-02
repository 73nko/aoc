use anyhow::Result;
use outcome::Outcome;
use play::Play;

mod outcome;
mod play;

fn solve(input: &[(Play, Play)]) -> Result<usize> {
    Ok(input.iter().map(Outcome::get_score).sum())
}

fn prepare_input_a(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|l| l.trim().split_once(' ').unwrap())
        .map(Play::get_plays)
        .collect()
}

fn prepare_input_b(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|l| l.trim().split_once(' ').unwrap())
        .map(Play::calculate_plays)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"A Y
    B X
    C Z"#;
    #[test]
    fn test_part1() {
        let input = prepare_input_a(INPUT);
        assert_eq!(solve(&input).unwrap(), 15);
    }
    #[test]
    fn test_part2() {
        let input = prepare_input_b(INPUT);
        assert_eq!(solve(&input).unwrap(), 12);
    }
}

fn main() -> Result<()> {
    println!(
        "PART 1: {:?}",
        solve(&prepare_input_a(include_str!("../input.txt")))?
    );
    println!(
        "PART 2: {:?}",
        solve(&prepare_input_b(include_str!("../input.txt")))?
    );
    Ok(())
}
