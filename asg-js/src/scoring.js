export const PLAYER_1 = Symbol("Player 1");
export const PLAYER_2 = Symbol("Player 2");

export function scoring(...args) {
    if (args.length === 0) return scoring({}, null);
    if (args.length === 1) return scoring({}, args[0]);
    return calculateState(...args);
}

const baseLineScores = state =>
    ({ [PLAYER_1]: 0, [PLAYER_2]: 0, ...state });

const describePlayerScore = numericScore =>
    ["love", "15", "30", "40"][numericScore];

const loveAll = state =>
    (state[PLAYER_1] + state[PLAYER_2] === 0) 
    && "love-all";

const deuce = state =>
    (state[PLAYER_1] > 2 && state[PLAYER_1] === state[PLAYER_2]) 
    && "Deuce";

const advantage = state => 
    state[PLAYER_1] > 2 && (Math.abs(state[PLAYER_1] - state[PLAYER_2]) === 1)
    && ((state[PLAYER_1] > state[PLAYER_2]) ? `Advantage ${PLAYER_1.description}` : `Advantage ${PLAYER_2.description}`);

const winner = state =>
    ([PLAYER_1, PLAYER_2]
        .filter(player => state[player] > 3)
        .map(winner => `Winner: ${winner.description}`))[0];

const underWay = state =>
    [state[PLAYER_1], state[PLAYER_2]]
        .map(describePlayerScore).join("-");

const describeScore = state =>
    ({ ...state, score: loveAll(state) || deuce(state) || advantage(state) || winner(state) || underWay(state) });

const addPoint = (state, point) =>
    ({ ...state, [point]: state[point] + 1 });

const calculateState = (previous, point) =>
    describeScore(addPoint(baseLineScores(previous), point));