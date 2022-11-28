use anyhow::Result;

fn part1(input: &Vec<i32>) -> Result<usize> {
    Ok(input
        .into_iter()
        .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > &prev { sum + 1 } else { sum }, *n)
        })
        .0)
}
fn part2(input: &Vec<i32>) -> Result<usize> {
    Ok(input
        .windows(3)
        .map(|n| n.iter().sum())
        .fold((0, i32::MAX), |(sum, prev), n| {
            (if n > prev { sum + 1 } else { sum }, n)
        })
        .0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 0);
    }
}

fn main() -> Result<()> {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("PART 1: {:?}", part1(&input)?);
    println!("PART 2: {:?}", part2(&input)?);
    Ok(())
}
