
namespace David;

public class GameTests
{
    [Fact]
    public void ShouldReturnLoveAllForANewGame()
    {
        var game = new Game();
        var result = game.GetStringScore();

        Assert.Equal("love-all", result);
    }

    [Theory]
    [InlineData(1, "15-love")]
    [InlineData(2, "30-love")]
    [InlineData(3, "40-love")]
    [InlineData(4, "game-player-one")]
    public void ShouldReturnLoveForANewGameAndOnePointsForPlayerOne(int points, string expectedScore)
    {
        var game = new Game();
        for (var i = 0; i < points; i++)
        {
            game.AddPlayerOneScore();
        }

        var result = game.GetStringScore();

        Assert.Equal(expectedScore, result);
    }

    [Theory]
    [InlineData(1, "love-15")]
    [InlineData(2, "love-30")]
    [InlineData(3, "love-40")]
    [InlineData(4, "game-player-two")]
    public void ShouldReturnLoveForANewGameAndPointsForPlayerTwo(int points, string expectedScore)
    {
        var game = new Game();
        for (var i = 0; i < points; i++)
        {
            game.AddPlayerTwoScore();
        }

        var result = game.GetStringScore();

        Assert.Equal(expectedScore, result);
    }

    [Theory]
    [InlineData(3, 3, "deuce")]
    [InlineData(4, 3, "advantage-player-one")]
    [InlineData(3, 4, "advantage-player-two")]
    [InlineData(4, 4, "deuce")]
    [InlineData(4, 5, "advantage-player-two")]
    [InlineData(5, 4, "advantage-player-one")]
    [InlineData(6, 4, "game-player-one")]
    [InlineData(4, 6, "game-player-two")]
    public void ShouldReturnExpectedScoreForBothPlayers(int pointsForA, int pointsForB, string expectedScore)
    {
        var game = new Game();
        var iterations = Math.Max(pointsForA, pointsForB);
        for (var i = 0; i < iterations; i++)
        {
            if (pointsForA > 0)
            {
                game.AddPlayerOneScore();
                pointsForA--;
            }

            if (pointsForB > 0)
            {
                game.AddPlayerTwoScore();
                pointsForB--;
            }
        }

        var result = game.GetStringScore();

        Assert.Equal(expectedScore, result);
    }

    [Theory]
    [InlineData(5)]
    public void ShouldReturnFalseWhenAddingPointsOverCompletedGameForPlayerOne(int points)
    {
        var game = new Game();
        bool? lastResult = null;
        for (var i = 0; i < points; i++)
        {
            lastResult = game.AddPlayerOneScore();
        }

        Assert.False(lastResult);
    }
    
    [Theory]
    [InlineData(5)]
    public void ShouldReturnFalseWhenAddingPointsOverCompletedGameForPlayerTwo(int points)
    {
        var game = new Game();
        bool? lastResult = null;
        for (var i = 0; i < points; i++)
        {
            lastResult = game.AddPlayerTwoScore();
        }

        Assert.False(lastResult);
    }
}