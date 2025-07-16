const Score = {
    LOVE: 0,
    FIFTEEN: 1,
    THIRTY: 2,
    FORTY: 3,
}

function startGame() {
    const game = {
        getScore,
        incrementScoreForPlayer,
        player1: Score.LOVE,
        player2: Score.LOVE,
    };

    function getScore() {
        // both player scores are equal
        if (game.player1 === game.player2) {
            if (game.player1 >= Score.FORTY) {
                return "deuce";
            }
            return `${scoreToString(game.player1)}-all`;
        }

        // winner is the player with the most score, when that player's score
        // is equal or above 4, and the winner has 2 points more than the loser
        if (Math.max(game.player1, game.player2) > Score.FORTY) {
            const absDiff = Math.abs(game.player1 - game.player2);
            if (absDiff > 1) {
                if (game.player1 > game.player2) {
                    return "Player 1 wins";
                } else {
                    return "Player 2 wins";
                }
            } else {
                if (game.player1 > game.player2) {
                    return "Advantage Player 1";
                } else {
                    return "Advantage Player 2";
                }
            }
        }

        // convert number to string
        let player1Score = scoreToString(game.player1);
        let player2Score = scoreToString(game.player2);

        // combine score strings
        return `${player1Score}-${player2Score}`;
    }

    function scoreToString(score) {
        switch (score) {
            case Score.LOVE:
                return "love";
            case Score.FIFTEEN:
                return "15";
            case Score.THIRTY:
                return "30";
            case Score.FORTY:
                return "40";
        }
    }

    function incrementScoreForPlayer(playerNumber) {
        switch (playerNumber) {
            case 1:
                game.player1 += 1;
                break;
            case 2:
                game.player2 += 1;
                break;
            default:
                console.error(`Oh no, player ${playerNumber} does not exist`);
        }

        return {...game};
    }

    return game;
}

module.exports = {
    startGame,
};
