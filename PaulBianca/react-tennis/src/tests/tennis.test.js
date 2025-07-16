const {
    startGame
} = require('../app/logic/tennis');

test('create game on start', () => {
    let game = startGame();
    expect(game).toBeTruthy();
    expect(game).toHaveProperty('player1');
    expect(game).toHaveProperty('player2');
    expect(game.player1).toEqual(0);
    expect(game.player2).toEqual(0);
})

test('players start on love', () => {
    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");
})

test('give score to player1', () => {
    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(1);

    expect(game.player1).toEqual(1);
})

test('print score range for player1', () => {
    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(1);

    expect(game.getScore()).toEqual("15-love");
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("30-love");
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("40-love");
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("Player 1 wins");
})

test('print score range for player2', () => {
    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(2);

    expect(game.getScore()).toEqual("love-15");
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("love-30");
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("love-40");
    game.incrementScoreForPlayer(2)
    expect(game.getScore()).toEqual("Player 2 wins");
})

test('print increase player2 by 2 and player1 by 4', () => {

    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(2);
    game.incrementScoreForPlayer(1);

    expect(game.getScore()).toEqual("15-all");
    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("40-30");
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("Player 1 wins");
})

test('print drawing all the way up', () => {

    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(2);
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("15-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("30-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");
})
test('print drawing all the way up', () => {

    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(2);
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("15-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("30-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");
});

test('Close match, Player 2 wins', () => {

    const game = startGame();

    const score = game.getScore();

    expect(score).toEqual("love-all");

    game.incrementScoreForPlayer(2);
    game.incrementScoreForPlayer(1);
    expect(game.getScore()).toEqual("15-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("30-all");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(1);
    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("deuce");

    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("Advantage Player 2");

    game.incrementScoreForPlayer(2);
    expect(game.getScore()).toEqual("Player 2 wins");

});
