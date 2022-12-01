use anyhow::{Ok, Result};

type ElvesCalories = Vec<Vec<usize>>;

fn part1(input: &ElvesCalories) -> Result<usize> {
    let result = input.iter().map(|elf| elf.iter().sum()).max();

    println!("{:?}", result);
    Ok(result.unwrap())
}

fn part2(input: &ElvesCalories) -> Result<usize> {
    let mut result: Vec<usize> = input.iter().map(|elf| elf.iter().sum()).collect();

    result.sort();

    let result = result.iter().rev().take(3).sum();

    Ok(result)
}

fn prepare_input(input: &str) -> ElvesCalories {
    input
        .split("\n\n")
        .map(|elf_cal| {
            elf_cal
                .lines()
                .map(|cals| cals.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
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
        assert_eq!(part1(&input).unwrap(), 24000);
    }
    #[test]
    fn test_part2() {
        let input = prepare_input(INPUT);
        assert_eq!(part2(&input).unwrap(), 45000);
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("PART 1: {:?}", part1(&prepare_input(input))?);
    println!("PART 2: {:?}", part2(&prepare_input(input))?);
    Ok(())
}
