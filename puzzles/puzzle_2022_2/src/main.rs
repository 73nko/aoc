use anyhow::Result;

enum Play {
    Rock,
    Paper,
    Scissors,
}

fn solve(input: &[(Play, Play)]) -> Result<usize> {
    Ok(input
        .iter()
        .map(|(elf_play, my_play)| get_score(elf_play, my_play))
        .sum())
}

fn prepare_input_a(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|l| l.trim().split_once(' ').unwrap())
        .map(|(elf_move, my_move): (&str, &str)| (get_play(elf_move), get_play(my_move)))
        .collect()
}

fn prepare_input_b(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|l| l.trim().split_once(' ').unwrap())
        .map(|(elf_move, result): (&str, &str)| calculate_plays(elf_move, result))
        .collect()
}

fn get_play(mov: &str) -> Play {
    match mov {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => panic!("unreachable string"),
    }
}

fn get_score(elf_play: &Play, my_play: &Play) -> usize {
    let my_play_score = match my_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };

    let result_score = match (my_play, elf_play) {
        (Play::Rock, Play::Scissors)
        | (Play::Paper, Play::Rock)
        | (Play::Scissors, Play::Paper) => 6,
        (Play::Rock, Play::Rock)
        | (Play::Paper, Play::Paper)
        | (Play::Scissors, Play::Scissors) => 3,
        _ => 0,
    };

    result_score + my_play_score
}

fn calculate_plays(elf_move: &str, result: &str) -> (Play, Play) {
    let elf_play = get_play(elf_move);
    let my_play = match result {
        "X" => get_lose(&elf_play),
        "Y" => get_draw(&elf_play),
        "Z" => get_win(&elf_play),
        _ => panic!("unreachable result"),
    };

    (elf_play, my_play)
}

fn get_lose(elf_play: &Play) -> Play {
    match elf_play {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
    }
}

fn get_draw(elf_play: &Play) -> Play {
    match elf_play {
        Play::Rock => Play::Rock,
        Play::Paper => Play::Paper,
        Play::Scissors => Play::Scissors,
    }
}

fn get_win(elf_play: &Play) -> Play {
    match elf_play {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
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
