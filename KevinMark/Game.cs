namespace KevinMark;

public class Game
{
    private Score _playerOneScore;
    private Score _playerTwoScore;

    public bool AddPlayerOneScore()
    {
        (var isValid, _playerOneScore, _playerTwoScore) = AddScore(_playerOneScore, _playerTwoScore);

        return isValid;
    }

    public bool AddPlayerTwoScore()
    {
        (var isValid, _playerTwoScore, _playerOneScore) = AddScore(_playerTwoScore, _playerOneScore);

        return isValid;
    }

    private string GetStringForScore(Score playerScore)
    {
        return playerScore switch
        {
            Score.Love => "love",
            Score.Fifteen => "15",
            Score.Thirty => "30",
            Score.Forty => "40",
            _ => throw new ArgumentOutOfRangeException(nameof(_playerOneScore), _playerOneScore, null)
        };
    }


    private (bool valid, Score currentPlayer, Score otherPlayer) AddScore(Score currentPlayer, Score otherPlayer)
    {
        return (currentPlayer, otherPlayer) switch
        {
            (Score.Game, _) or (_, Score.Game) => (false, currentPlayer, otherPlayer),
            (Score.Forty, < Score.Forty) => (true, Score.Game, otherPlayer),
            (Score.Forty, Score.Forty) => (true, Score.Advantage, otherPlayer),
            (Score.Forty, Score.Advantage) => (true, currentPlayer, Score.Forty),
            _ => (true, ++currentPlayer, otherPlayer)
        };
    }


    public string GetStringScore()
    {
        var result = (_playerOneScore, _playerTwoScore) switch
        {
            (Score.Game, _) => "game-player-one",
            (_, Score.Game) => "game-player-two",
            (Score.Advantage, _) => "advantage-player-one",
            (_, Score.Advantage) => "advantage-player-two",
            (Score.Love, Score.Love) => "love-all",
            (Score.Forty, Score.Forty) => "deuce",
            _ => $"{GetStringForScore(_playerOneScore)}-{GetStringForScore(_playerTwoScore)}"
        };

        return result;
    }
}