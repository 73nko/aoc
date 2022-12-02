use crate::play::Move;

pub enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Outcome {
    fn match_result((elf_play, my_play): &Move) -> Outcome {
        if elf_play == my_play {
            return Outcome::Draw;
        } else if my_play.is_winner(elf_play) {
            return Outcome::Win;
        }

        Outcome::Lose
    }

    pub fn get_score(plays: &Move) -> usize {
        Self::match_result(plays) as usize + plays.1 as usize
    }
}
