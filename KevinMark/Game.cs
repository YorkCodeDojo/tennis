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
        if (_playerOneScore == Score.Game || _playerTwoScore == Score.Game)
        {
            return (false, currentPlayer, otherPlayer);
        }

        if (currentPlayer == Score.Forty)
        {
            if (otherPlayer < Score.Forty)
                return (true, Score.Game, otherPlayer);

            if (otherPlayer == Score.Forty)
                return (true, Score.Advantage, otherPlayer);

            if (otherPlayer == Score.Advantage)
                return (true, currentPlayer, --otherPlayer);
        }

        return (true, ++currentPlayer, otherPlayer);
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