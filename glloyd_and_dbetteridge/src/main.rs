use tennis::Game;

fn main() {
    let mut game = Game::new();
    println!("Starting game: {}", game.print_score());
    
    game.point_for_player_one();
    println!("After player 1 scores: {}", game.print_score());
    
    game.point_for_player_two();
    println!("After player 2 scores: {}", game.print_score());
}
