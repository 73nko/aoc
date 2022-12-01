use anyhow::{Ok, Result};

type ElfCalories = Vec<usize>;

fn solve(input: &ElfCalories, elves: usize) -> Result<usize> {
    Ok(input.iter().take(elves).sum())
}

fn prepare_input(input: &str) -> ElfCalories {
    let mut formatted_input: ElfCalories = input
        .split("\n\n")
        .map(|elf_cal| {
            elf_cal
                .lines()
                .map(|cals| cals.trim().parse::<usize>().unwrap())
                .collect()
        })
        .map(|elf: Vec<usize>| elf.iter().sum())
        .collect();

    formatted_input.sort();
    formatted_input.reverse();

    formatted_input
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000"#;
    #[test]
    fn test_part1() {
        let input = prepare_input(INPUT);
        assert_eq!(solve(&input, 1).unwrap(), 24000);
    }
    #[test]
    fn test_part2() {
        let input = prepare_input(INPUT);
        assert_eq!(solve(&input, 3).unwrap(), 45000);
    }
}

fn main() -> Result<()> {
    let input = prepare_input(include_str!("../input.txt"));
    println!("PART 1: {:?}", solve(&input, 1)?);
    println!("PART 2: {:?}", solve(&input, 3)?);
    Ok(())
}
