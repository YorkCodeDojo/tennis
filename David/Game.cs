namespace David;

public class Game
{
    private record State
    {
        public required int NextPlayer1State { get; init; }
        public required int NextPlayer2State { get; init; }
        public required string Text { get; init; }
    }

    private int _currentState;
    
    private readonly State[] _states =
    [
        new() { NextPlayer1State = 1, NextPlayer2State = 5, Text = "love-all" }, // 0
        new() { NextPlayer1State = 2, NextPlayer2State = 9, Text = "15-love" }, // 1
        new() { NextPlayer1State = 3, NextPlayer2State = 10, Text = "30-love" }, // 2
        new() { NextPlayer1State = 4, NextPlayer2State = 11, Text = "40-love" }, // 3
        new() { NextPlayer1State = 4, NextPlayer2State = 4, Text = "game-player-one" }, // 4
        new() { NextPlayer1State = 9, NextPlayer2State = 6, Text = "love-15" }, // 5
        new() { NextPlayer1State = 13, NextPlayer2State = 7, Text = "love-30" }, // 6
        new() { NextPlayer1State = 15, NextPlayer2State = 8, Text = "love-40" }, // 7
        new() { NextPlayer1State = 8, NextPlayer2State = 8, Text = "game-player-two" }, // 8
        new() { NextPlayer1State = 10, NextPlayer2State = 12, Text = "15-all" }, // 9
        new() { NextPlayer1State = 11, NextPlayer2State = 13, Text = "30-15" }, // 10
        new() { NextPlayer1State = 4, NextPlayer2State = 14, Text = "40-15" }, // 11
        new() { NextPlayer1State = 13, NextPlayer2State = 15, Text = "15-30" }, // 12
        new() { NextPlayer1State = 14, NextPlayer2State = 19, Text = "30-all" }, // 13
        new() { NextPlayer1State = 4, NextPlayer2State = 16, Text = "40-30" }, // 14
        new() { NextPlayer1State = 19, NextPlayer2State = 8, Text = "15-40" }, // 15
        new() { NextPlayer1State = 17, NextPlayer2State = 18, Text = "deuce" }, // 16
        new() { NextPlayer1State = 4, NextPlayer2State = 16, Text = "advantage-player-one" }, // 17
        new() { NextPlayer1State = 16, NextPlayer2State = 8, Text = "advantage-player-two" }, // 18
        new() { NextPlayer1State = 17, NextPlayer2State = 8, Text = "30-40" }, // 19
    ];

    public string GetStringScore()
    {
        return _states[_currentState].Text;
    }

    public bool AddPlayerOneScore()
    {
        var nextState = _states[_currentState].NextPlayer1State;
        var previousState = _currentState;
        _currentState = nextState;
        return nextState != previousState;
    }

    public bool AddPlayerTwoScore()
    {
        var nextState = _states[_currentState].NextPlayer2State;
        var previousState = _currentState;
        _currentState = nextState;
        return nextState != previousState;
    }
}