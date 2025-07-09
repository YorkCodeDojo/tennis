fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
enum GameState{
    eLoveAll,
    e15Love,
    e30Love,
    eLove15
}

struct Game {
    state: GameState
}

impl Game {
    pub(crate) fn point_for_player_two(&mut self) {
        self.state = match self.state {
            _ => GameState::eLove15,
        }
    }
}

impl Game {

    fn new() -> Game {
        Game {
            state: GameState::eLoveAll
        }
    }
    pub(crate) fn point_for_player_one(&mut self) {
        if self.state == GameState::eLoveAll
        {
            self.state = GameState::e15Love;
        }
        else {
            self.state = GameState::e30Love;
        }
    }

    pub(crate) fn print_score(&self) -> String {
         match self.state {
             GameState::eLoveAll => "love-all",
             GameState::e15Love => "15-love",
             GameState::e30Love => "30-love",
             GameState::eLove15 => "love-15",
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
}
