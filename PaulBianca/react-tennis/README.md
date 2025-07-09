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
* 


This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.js`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
