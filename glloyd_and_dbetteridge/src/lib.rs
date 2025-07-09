use std::fmt::Display;

#[derive(PartialEq, Copy, Clone)]
enum PlayerScore {
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
    pub fn new() -> Game {
        Game::Scores(PlayerScore::Love, PlayerScore::Love)
    }

    pub fn advance_score(self, player: Player) -> Game {
        match (player, self) {
            (Player::One, Game::Deuce) => Game::AdvantagePlayer1,
            (Player::Two, Game::Deuce) => Game::AdvantagePlayer2,

            (Player::One, Game::AdvantagePlayer2) => Game::Deuce,
            (Player::Two, Game::AdvantagePlayer1) => Game::Deuce,
            (Player::One, Game::Scores(PlayerScore::Thirty, PlayerScore::Forty)) => Game::Deuce,
            (Player::Two, Game::Scores(PlayerScore::Forty, PlayerScore::Thirty)) => Game::Deuce,

            (Player::One, Game::AdvantagePlayer1) => Game::Player1Win,
            (Player::Two, Game::AdvantagePlayer2) => Game::Player2Win,
            (Player::One, Game::Scores(PlayerScore::Forty, _)) => Game::Player1Win,
            (Player::Two, Game::Scores(_, PlayerScore::Forty)) => Game::Player2Win,

            (Player::One, Game::Scores(player1, player2)) => {
                Game::Scores(player1.advance_score(), player2)
            }
            (Player::Two, Game::Scores(player1, player2)) => {
                Game::Scores(player1, player2.advance_score())
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
            Game::Deuce => "Deuce".to_string(),
            Game::AdvantagePlayer1 => "Advantage player1".to_string(),
            Game::AdvantagePlayer2 => "Advantage player2".to_string(),
            Game::Player1Win => "player1 has won".to_string(),
            Game::Player2Win => "player2 has won".to_string(),
            Game::Scores(lhs, rhs) if lhs == rhs => format!("{}-all", lhs),
            Game::Scores(lhs, rhs) => format!("{}-{}", lhs, rhs),
        }
        .into()
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