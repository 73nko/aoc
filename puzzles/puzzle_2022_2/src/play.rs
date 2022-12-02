#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn new(mov: &str) -> Self {
        match mov {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => panic!("unreachable string"),
        }
    }

    fn from_score(score: usize) -> Self {
        match score {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("Forbidden."),
        }
    }

    pub fn get_plays((elf_move, my_move): (&str, &str)) -> (Play, Play) {
        (Play::new(elf_move), Play::new(my_move))
    }

    pub fn calculate_plays((elf_move, result): (&str, &str)) -> (Play, Play) {
        let elf_play = Play::new(elf_move);
        let my_play = match result {
            "X" => Play::from_score((elf_play as usize + 1) % 3),
            "Y" => elf_play,
            "Z" => Play::from_score((elf_play as usize + 3) % 3),
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
