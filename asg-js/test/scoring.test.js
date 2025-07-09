import t from "tap";
import { PLAYER_1, PLAYER_2, scoring } from "../src/scoring.js";

function accumulatePoints(...args) {
    return args.reduce((prev, x) => scoring(prev, x), scoring());
}

function expect(...args) {

    const expected = args.pop();
    return async t => t.equal(accumulatePoints(...args)?.score, expected);

}

await t.test("When a game is new", async t => {

    await t.test("The score is love-all", expect("love-all"));

    await t.test("When player 1 scores a point", async t => {

        await t.test("The score is 15-love", expect(
            PLAYER_1,
            "15-love"));
        
        await t.test("And a 2nd point", async t => {

            await t.test("The score is 30-love", expect(
                PLAYER_1, PLAYER_1,
                "30-love"));
            

            await t.test("And a 3rd point", async t => {

                await t.test("The score is 40-love", expect(
                    PLAYER_1, PLAYER_1, PLAYER_1, 
                    "40-love"));

                await t.test("And a 4th, winning point", async t => {

                    t.test("Player 1 wins", expect(
                        PLAYER_1, PLAYER_1, PLAYER_1, PLAYER_1, 
                        "Winner: Player 1"));

                });

                await t.test("But player 2 also scores 3 points", async t => {

                    t.test("The score is deuce", expect(
                        PLAYER_1, PLAYER_1, PLAYER_1, PLAYER_2, PLAYER_2, PLAYER_2,
                        "Deuce"));

                    await t.test("And player 1 scores a 4th point", async t => {

                        t.test("The score is Advantage Player 1", expect(
                            PLAYER_1, PLAYER_1, PLAYER_1, PLAYER_2, PLAYER_2, PLAYER_2, PLAYER_1,
                            "Advantage Player 1"));
                        
                        await t.test("And Player 1 scores a 5th, winning point", expect(
                            PLAYER_1, PLAYER_1, PLAYER_1, PLAYER_2, PLAYER_2, PLAYER_2, PLAYER_1, PLAYER_1,
                            "Winner: Player 1"));

                        await t.test("But Player 2 wins a 4th point", async t => {

                            t.test("The score is deuce", expect(
                                PLAYER_1, PLAYER_1, PLAYER_1, PLAYER_2, PLAYER_2, PLAYER_2, PLAYER_1, PLAYER_2,
                                "Deuce"));

                        });

                    });

                });

            });

        });

    });

});
