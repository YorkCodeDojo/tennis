from main import Game
def test_func():
    assert True

def test_new_game():
    game = Game()
    assert game.PrintScore() == "love-all"

def test_new_game_playerOne_scores():
    game = Game()
    game._playerOneScores()
    assert game.PrintScore() == "15-love"

def test_new_game_playerTwo_scores():
    game = Game()
    game._playerTwoScores()
    assert game.PrintScore() == "love-15"

def test_playerOne_scores_twice():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    assert game.PrintScore() == "30-love"

def test_playerTwo_scores_twice():
    game = Game()
    game._playerTwoScores()
    game._playerTwoScores()
    assert game.PrintScore() == "love-30"

def test_playerOne_scores_three_times():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    assert game.PrintScore() == "40-love"

def test_playerTwo_scores_three_times():
    game = Game()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    assert game.PrintScore() == "love-40"

def test_playerOne_scores_four_times():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    assert game.PrintScore() == "PlayerOne wins"

def test_playerTwo_scores_four_times():
    game = Game()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    assert game.PrintScore() == "PlayerTwo wins"

def test_both_players_score_40():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    assert game.PrintScore() == "deuce"

def test_playerOne_scores_after_deuce():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerOneScores()  # Player One scores after deuce
    assert game.PrintScore() == "advantage PlayerOne"

def test_playerTwo_scores_after_deuce():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()  # Player Two scores after deuce
    assert game.PrintScore() == "advantage PlayerTwo"

def test_playertwo_scores_after_playerone_advantage():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerOneScores()  # playerOne advantge
    game._playerTwoScores()  # Player Two scores after Player One's advantage
    assert game.PrintScore() == "deuce"

def test_playerOne_scores_twice_after_duce():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerOneScores()  # Player One scores after deuce
    game._playerOneScores()  # Player One scores again
    assert game.PrintScore() == "PlayerOne wins"

def test_playerTwo_scores_twice_after_duce():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()  # Player Two scores after deuce
    game._playerTwoScores()  # Player Two scores again
    assert game.PrintScore() == "PlayerTwo wins"

def test_playerTwo_scores_after_playerOne_second_advantage():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerOneScores()  # Player One advantage
    game._playerTwoScores()  # Player Two scores after Player One's advantage
    game._playerOneScores()  # Player One second advantage
    game._playerTwoScores()  # Player Two scores after Player One's second advantage
    assert game.PrintScore() == "deuce"

def test_playerOne_scores_after_playerTwo_second_advantage():
    game = Game()
    game._playerOneScores()
    game._playerOneScores()
    game._playerOneScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()
    game._playerTwoScores()  # Player Two advantage
    game._playerOneScores()  # Player One scores after Player Two's advantage
    game._playerTwoScores()  # Player Two second advantage
    game._playerOneScores()  # Player One scores after Player Two's second advantage
    assert game.PrintScore() == "deuce"