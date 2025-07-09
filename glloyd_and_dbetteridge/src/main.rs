use tennis::{Game, Player};

fn main() {
    let mut game = Game::default();
    println!("Starting game: {}", game);
    
    game.score_point(Player::One);
    println!("After player 1 scores: {}", game);
    
    game.score_point(Player::Two);
    println!("After player 2 scores: {}", game);
}
