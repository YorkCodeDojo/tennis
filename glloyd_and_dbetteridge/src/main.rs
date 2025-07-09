use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Copy, Clone)]
enum PlayerScore {
    Love,
    Fifteen,
    Thirty,
    Forty,
    Win,
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Love => "love",
            Self::Fifteen => "15",
            Self::Thirty => "30",
            Self::Forty => "40",
            Self::Win => "player1 has won",
        };
        write!(f, "{}", str)
    }
}

impl PlayerScore {
    pub(crate) fn advance_score(self) -> Self {
        match self {
            Self::Love => Self::Fifteen,
            Self::Fifteen => Self::Thirty,
            Self::Thirty => Self::Forty,
            Self::Forty => Self::Win,
            Self::Win => Self::Win,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Game {
    Deuce,
    AdvantagePlayer1,
    AdvantagePlayer2,
    Scores(PlayerScore, PlayerScore),
}

#[derive(PartialEq, Copy, Clone)]
enum Player {
    One,
    Two,
}

impl Game {
    fn new() -> Game {
        Game::Scores(PlayerScore::Love, PlayerScore::Love)
    }

    pub(crate) fn advance_score(self, player: Player) -> Game {
        match player {
            Player::One => match self {
                Game::Deuce => Game::AdvantagePlayer1,
                Game::AdvantagePlayer1 => Game::AdvantagePlayer1,
                Game::AdvantagePlayer2 => Game::AdvantagePlayer2,
                Game::Scores(player1, player2) => match (player1, player2) {
                    (PlayerScore::Win, _) => Game::Scores(player1, player2),
                    (_, PlayerScore::Win) => Game::Scores(player1, player2),
                    (PlayerScore::Thirty, PlayerScore::Forty) => Game::Deuce,
                    _ => Game::Scores(player1.advance_score(), player2),
                },
            },
            Player::Two => match self {
                Game::Deuce => Game::AdvantagePlayer2,
                Game::AdvantagePlayer1 => Game::AdvantagePlayer1,
                Game::AdvantagePlayer2 => Game::AdvantagePlayer2,
                Game::Scores(player1, player2) => match (player1, player2) {
                    (PlayerScore::Win, _) => Game::Scores(player1, player2),
                    (_, PlayerScore::Win) => Game::Scores(player1, player2),
                    (PlayerScore::Forty, PlayerScore::Thirty) => Game::Deuce,
                    _ => Game::Scores(player1, player2.advance_score()),
                },
            },
        }
    }

    pub(crate) fn point_for_player_one(&mut self) {
        *self = self.advance_score(Player::One);
    }

    pub(crate) fn point_for_player_two(&mut self) {
        *self = self.advance_score(Player::Two);
    }

    pub(crate) fn print_score(&self) -> String {
        match self {
            Game::Deuce => "Deuce".to_string(),
            Game::AdvantagePlayer1 => "Advantage player1".to_string(),
            Game::AdvantagePlayer2 => "Advantage player2".to_string(),
            Game::Scores(PlayerScore::Win, _) => "player1 has won".to_string(),
            Game::Scores(_, PlayerScore::Win) => "player2 has won".to_string(),
            Game::Scores(lhs, rhs) if lhs == rhs => format!("{}-all", lhs),
            Game::Scores(lhs, rhs) => format!("{}-{}", lhs, rhs),
        }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn a_new_game_is_love_all() {
        let sut = Game::new();
        assert_eq!(sut.print_score(), "love-all");
    }

    #[test]
    fn if_player1_once_then_has_score_15_love() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        assert_eq!(sut.print_score(), "15-love");
    }

    #[test]
    fn if_player1_scores_twice_then_has_score_30_love() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        sut.point_for_player_one();
        assert_eq!(sut.print_score(), "30-love");
    }

    #[test]
    fn if_player2_once_then_has_score_love_15() {
        let mut sut = Game::new();
        sut.point_for_player_two();
        assert_eq!(sut.print_score(), "love-15");
    }

    #[test]
    fn if_player1_scores_three_times_then_has_score_40_love() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        sut.point_for_player_one();
        sut.point_for_player_one();
        assert_eq!(sut.print_score(), "40-love");
    }

    #[test]
    fn if_player1_scores_four_times_then_has_player1_has_won() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        sut.point_for_player_one();
        sut.point_for_player_one();
        sut.point_for_player_one();
        assert_eq!(sut.print_score(), "player1 has won");
    }

    #[test]
    fn if_both_players_have_40_then_its_deuce() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        sut.point_for_player_one();
        sut.point_for_player_one();

        sut.point_for_player_two();
        sut.point_for_player_two();
        sut.point_for_player_two();

        assert_eq!(sut.print_score(), "Deuce");
    }

    #[test]
    fn from_deuce_player1_can_take_the_advantage() {
        let mut sut = Game::new();
        sut.point_for_player_one();
        sut.point_for_player_one();
        sut.point_for_player_one();

        sut.point_for_player_two();
        sut.point_for_player_two();
        sut.point_for_player_two();

        sut.point_for_player_one();

        assert_eq!(sut.print_score(), "Advantage player1");
    }
}
