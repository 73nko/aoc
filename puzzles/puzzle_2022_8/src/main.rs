use itertools::Itertools;
use std::cmp::max;

type Grid = Vec<Vec<u8>>;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn examine_tree(grid: &[Vec<u8>], r: usize, c: usize) -> (bool, usize) {
    let tree = grid[r][c];
    let (mut invisible, mut score) = (true, 1);

    for (dr, dc) in DIRS {
        let (mut r, mut c, mut i, mut visible) = (r, c, 0, true);

        while let Some(&next) = grid
            .get((r as isize + dr) as usize)
            .and_then(|x| x.get((c as isize + dc) as usize))
        {
            i += 1;
            if tree <= next {
                visible = false;
                break;
            }
            r = (r as isize + dr) as usize;
            c = (c as isize + dc) as usize;
        }
        if visible {
            invisible = false;
        }
    }
    (!invisible, score)
}

fn solve_parts(grid: Grid) -> (usize, usize) {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .fold((0, 0), |(p1, p2), (r, c)| {
            let (visible, score) = examine_tree(&grid, r, c);
            (p1 + visible as usize, max(p2, score))
        })
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"30373
    25512
    65332
    33549
    35390"#;
    #[test]
    fn test_part1() {
        assert_eq!(solve_parts(parse_input(INPUT)), (21, 8));
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);
    println!("PART 1: {:?}", solve_parts(input));
    // println!("PART 2: {:?}", part2(input));
}
