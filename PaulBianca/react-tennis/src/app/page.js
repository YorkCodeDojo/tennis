'use client';

const tennis = require('./logic/tennis');
import {useState} from "react";

export default function Home() {
    const [game, setGame] = useState(null);

    return (
        <div
            className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
            <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">
                {!game ? <button onClick={() => {
                    setGame(tennis.startGame());
                }}>Start Game</button> : <Game gameState={[game, setGame]}/>}
            </main>
        </div>
    );
}

function Game({gameState: [game, setGame]}) {
    return <div>
        <div>
            <button onClick={() => setGame(tennis.startGame())}>New game</button>
            <br/>
            <br/>
            Score: {game.getScore()}
            <br/>
            <br/>
            <button onClick={() => {
                setGame(game.incrementScoreForPlayer(1));
            }}>Add Score to Player 1
            </button>
            <br/>
            <br/>
            <button onClick={() => {
                setGame(game.incrementScoreForPlayer(2));
            }}>Add Score to Player 2
            </button>
        </div>
    </div>;
}
