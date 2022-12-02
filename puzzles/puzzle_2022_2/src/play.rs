#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub type Move = (Play, Play);

impl Play {
    fn new(mov: &str) -> Self {
        match mov {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => panic!("unreachable string"),
        }
    }

    fn from_result(score: usize) -> Self {
        match score {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("Forbidden."),
        }
    }

    pub fn get_plays((elf_move, my_move): (&str, &str)) -> Move {
        (Play::new(elf_move), Play::new(my_move))
    }

    pub fn calculate_plays((elf_move, result): (&str, &str)) -> Move {
        let elf_play = Play::new(elf_move);
        let my_play = match result {
            "X" => Play::from_result((elf_play as usize + 1) % 3),
            "Y" => Play::from_result((elf_play as usize + 2) % 3),
            "Z" => Play::from_result((elf_play as usize + 3) % 3),
            _ => panic!("unreachable result"),
        };

        (elf_play, my_play)
    }

    pub fn is_winner(self, play: &Play) -> bool {
        matches!(
            (self, *play),
            (Play::Rock, Play::Scissors)
                | (Play::Paper, Play::Rock)
                | (Play::Scissors, Play::Paper)
        )
    }
}
