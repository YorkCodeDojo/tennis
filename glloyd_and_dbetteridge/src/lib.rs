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
    AdvantagePlayer1,
    AdvantagePlayer2,
    Player1Win,
    Player2Win,
    Scores(PlayerScore, PlayerScore),
}

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    One,
    Two,
}

impl Game {
    pub fn new() -> Self {
        Self::Scores(PlayerScore::Love, PlayerScore::Love)
    }

    pub fn advance_score(self, player: Player) -> Self {
        match (player, self) {
            (Player::One, Self::AdvantagePlayer2) => Self::Deuce,
            (Player::Two, Self::AdvantagePlayer1) => Self::Deuce,
            (Player::One, Self::Scores(PlayerScore::Thirty, PlayerScore::Forty)) => Self::Deuce,
            (Player::Two, Self::Scores(PlayerScore::Forty, PlayerScore::Thirty)) => Self::Deuce,

            (Player::One, Self::Deuce) => Self::AdvantagePlayer1,
            (Player::Two, Self::Deuce) => Self::AdvantagePlayer2,

            (Player::One, Self::AdvantagePlayer1) => Self::Player1Win,
            (Player::Two, Self::AdvantagePlayer2) => Self::Player2Win,
            (Player::One, Self::Scores(PlayerScore::Forty, _)) => Self::Player1Win,
            (Player::Two, Self::Scores(_, PlayerScore::Forty)) => Self::Player2Win,

            (Player::One, Self::Scores(player1, player2)) => {
                Self::Scores(player1.advance_score(), player2)
            }
            (Player::Two, Self::Scores(player1, player2)) => {
                Self::Scores(player1, player2.advance_score())
            }
            (_, x) => x,
        }
    }

    pub fn point_for_player_one(&mut self) {
        *self = self.advance_score(Player::One);
    }

    pub fn point_for_player_two(&mut self) {
        *self = self.advance_score(Player::Two);
    }

    pub fn print_score(&self) -> String {
        match self {
            Self::Deuce => "Deuce".to_string(),
            Self::AdvantagePlayer1 => "Advantage player1".to_string(),
            Self::AdvantagePlayer2 => "Advantage player2".to_string(),
            Self::Player1Win => "player1 has won".to_string(),
            Self::Player2Win => "player2 has won".to_string(),
            Self::Scores(lhs, rhs) if lhs == rhs => format!("{}-all", lhs),
            Self::Scores(lhs, rhs) => format!("{}-{}", lhs, rhs),
        }
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
    #[case(vec!(4,0), "player1 has won")]
    #[case(vec!(3,3), "Deuce")]
    #[case(vec!(3,3,1), "Advantage player1")]
    #[case(vec!(3,3,2), "player1 has won")]
    #[case(vec!(3,3,1,1), "Deuce")]
    fn test_tennis_scoring(#[case] sequence: Vec<i32>, #[case] expected: &str) {
        let mut game = Game::new();

        for (i, &n) in sequence.iter().enumerate() {
            if i % 2 == 0 {
                for _ in 0..n {
                    game.point_for_player_one();
                }
            } else {
                for _ in 0..n {
                    game.point_for_player_two();
                }
            }
        }

        assert_eq!(game.print_score(), expected);
    }
}
