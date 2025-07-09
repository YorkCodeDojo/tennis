use std::fmt::Display;

#[derive(PartialEq, Copy, Clone)]
pub enum PlayerScore {
    Love,
    Fifteen,
    Thirty,
    Forty,
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Love => "love",
            Self::Fifteen => "15",
            Self::Thirty => "30",
            Self::Forty => "40",
        };
        write!(f, "{}", str)
    }
}

impl PlayerScore {
    fn advance_score(self) -> Self {
        match self {
            Self::Love => Self::Fifteen,
            Self::Fifteen => Self::Thirty,
            Self::Thirty => Self::Forty,
            Self::Forty => Self::Forty,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Game {
    Deuce,
    Advantage(Player),
    Win(Player),
    Scores(PlayerScore, PlayerScore),
}

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    One,
    Two,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Player::One => "Player 1",
            Player::Two => "Player 2",
        };
        write!(f, "{}", str)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::Scores(PlayerScore::Love, PlayerScore::Love)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let score_text = match self {
            Self::Deuce => "Deuce",
            Self::Advantage(player) => return write!(f, "Advantage {}", player),
            Self::Win(player) => return write!(f, "{} has won", player),
            Self::Scores(lhs, rhs) if lhs == rhs => return write!(f, "{}-all", lhs),
            Self::Scores(lhs, rhs) => return write!(f, "{}-{}", lhs, rhs),
        };
        write!(f, "{}", score_text)
    }
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance_score(self, player: Player) -> Self {
        match (player, self) {
            (x, Self::Advantage(y)) if x != y => Self::Deuce,
            (Player::One, Self::Scores(PlayerScore::Thirty, PlayerScore::Forty)) => Self::Deuce,
            (Player::Two, Self::Scores(PlayerScore::Forty, PlayerScore::Thirty)) => Self::Deuce,

            (x, Self::Deuce) => Self::Advantage(x),

            (x, Self::Advantage(y)) if x == y => Self::Win(x),
            (Player::One, Self::Scores(PlayerScore::Forty, _)) => Self::Win(Player::One),
            (Player::Two, Self::Scores(_, PlayerScore::Forty)) => Self::Win(Player::Two),

            (Player::One, Self::Scores(player1, player2)) => {
                Self::Scores(player1.advance_score(), player2)
            }
            (Player::Two, Self::Scores(player1, player2)) => {
                Self::Scores(player1, player2.advance_score())
            }
            (_, x) => x,
        }
    }

    pub fn score_point(&mut self, player: Player) {
        *self = self.advance_score(player);
    }

}

#[cfg(test)]
mod tests {
    use crate::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!(0,0), "love-all")]
    #[case(vec!(1,0), "15-love")]
    #[case(vec!(2,0), "30-love")]
    #[case(vec!(0,1), "love-15")]
    #[case(vec!(3,0), "40-love")]
    #[case(vec!(4,0), "Player 1 has won")]
    #[case(vec!(3,3), "Deuce")]
    #[case(vec!(3,3,1), "Advantage Player 1")]
    #[case(vec!(3,3,2), "Player 1 has won")]
    #[case(vec!(3,3,1,1), "Deuce")]
    fn test_tennis_scoring(#[case] sequence: Vec<i32>, #[case] expected: &str) {
        let mut game = Game::new();

        for (i, &n) in sequence.iter().enumerate() {
            let player = if i % 2 == 0 { Player::One } else { Player::Two };
            for _ in 0..n {
                game.score_point(player);
            }
        }

        assert_eq!(game.to_string(), expected);
    }
}
