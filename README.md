# Tennis

## Rules

* Each player can have these points:  “love” “15” “30” “40”
* If you have 40 and you win the point you win the game, however there are special rules.
* If both have 40 the players are “deuce”.
* If the game is in deuce, the winner of a point will have advantage
* If the player with advantage wins the ball he wins the game
* If the player without advantage wins they are back at deuce.

## Task

Write a program which allows you to keep track of the scores during a game of tennis.

For example

```
game = new Game()
game.PrintScore();   // Displays love-all
game.PointForPlayerOne();
game.PrintScore();   // Displays 15-love
etc
```

## Testing
I suggest trying the ping-pong style of testing.  

* One person writes a failing test
* The other person then writes enough code to pass the test
* As a pair you then refactor
* Swap roles

## Advanced

If you get to the end of the task,  then try creating a 2nd implementation (without changing your tests).

For example,  try creating a version which doesn't use any IF statements.


## Getting started
1. Fork this repo (https://github.com/YorkCodeDojo/tennis) and clone it to your machine.
2. Create a folder in your local copy with name of your pair.
3. Put your code into this folder
4. At the end of the evening, create a pull request to merge your code back into the original repo


## References

* https://codingdojo.org/kata/Tennis/
* https://github.com/emilybache/Tennis-Refactoring-Kata/tree/main/csharp