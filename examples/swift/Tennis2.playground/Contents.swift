import UIKit

// Tennis game states
enum GameState {
    case playing
    case deuce
    case won
}

// Tennis points: 0, 15, 30, 40, advantage
enum Points {
    case love      // 0 points
    case fifteen   // 15 points
    case thirty    // 30 points
    case forty     // 40 points
    case advantage // advantage (after deuce)
}

// A tennis player with their current points
struct Player {
    let points: Points
    
    init(points: Points = .love) {
        self.points = points
    }
}

// The tennis game
struct Game {
    let state: GameState
    let player1: Player
    let player2: Player
    
    init(state: GameState = .playing, player1: Player = Player(), player2: Player = Player()) {
        self.state = state
        self.player1 = player1
        self.player2 = player2
    }
}

// Simple tennis scoring
struct TennisScorer {
    
    // Award a point to player 1 or player 2
    static func awardPoint(to playerNumber: Int, game: Game) -> Game {
        
        if playerNumber == 1 {
            let newPlayer1 = getNextPoints(from: game.player1.points)
            let updatedGame = Game(state: game.state,
                                 player1: Player(points: newPlayer1),
                                 player2: game.player2)
            return checkGameResult(updatedGame, whoScored: 1)
        } else {
            let newPlayer2 = getNextPoints(from: game.player2.points)
            let updatedGame = Game(state: game.state,
                                 player1: game.player1,
                                 player2: Player(points: newPlayer2))
            return checkGameResult(updatedGame, whoScored: 2)
        }
    }
    
    // What happens when you score a point?
    static func getNextPoints(from currentPoints: Points) -> Points {
        switch currentPoints {
        case .love:      return .fifteen
        case .fifteen:   return .thirty
        case .thirty:    return .forty
        case .forty:     return .advantage
        case .advantage: return .advantage // Stay at advantage until you win
        }
    }
    
    // Check if someone won or if it's deuce
    static func checkGameResult(_ game: Game, whoScored: Int) -> Game {
        let p1 = game.player1.points
        let p2 = game.player2.points
        
        // Both have 40? That's deuce!
        if p1 == .forty && p2 == .forty {
            return Game(state: .deuce, player1: game.player1, player2: game.player2)
        }
        
        // Someone with advantage wins the game
        if (whoScored == 1 && p1 == .advantage && p2 != .forty) ||
           (whoScored == 2 && p2 == .advantage && p1 != .forty) {
            print("🎾 Player \(whoScored) wins the game!")
            return Game(state: .won, player1: game.player1, player2: game.player2)
        }
        
        // Someone wins without deuce (opponent has less than 40)
        if (whoScored == 1 && p1 == .advantage && p2 != .forty && p2 != .advantage) ||
           (whoScored == 2 && p2 == .advantage && p1 != .forty && p1 != .advantage) {
            print("🎾 Player \(whoScored) wins the game!")
            return Game(state: .won, player1: game.player1, player2: game.player2)
        }
        
        // Back to deuce from advantage
        if game.state == .deuce &&
           ((whoScored == 1 && p2 == .advantage) || (whoScored == 2 && p1 == .advantage)) {
            return Game(state: .deuce,
                       player1: Player(points: .forty),
                       player2: Player(points: .forty))
        }
        
        return Game(state: .playing, player1: game.player1, player2: game.player2)
    }
}

// Show the score in a friendly way
struct ScoreDisplay {
    static func show(_ game: Game) {
        let p1Score = friendlyScore(game.player1.points)
        let p2Score = friendlyScore(game.player2.points)
        
        print("Score: \(p1Score) - \(p2Score)")
        if game.state == .deuce {
            print("It's DEUCE! 🤝")
        } else if game.state == .won {
            print("Game Over! 🏆")
        }
    }
    
    static func friendlyScore(_ points: Points) -> String {
        switch points {
        case .love:      return "0"
        case .fifteen:   return "15"
        case .thirty:    return "30"
        case .forty:     return "40"
        case .advantage: return "ADV"
        }
    }
}


// 🧪 SIMPLE TESTS - Easy to read and understand!

func testBasicScoring() {
    print("\n🏸 Testing Basic Tennis Scoring...")
    var game = Game()
    
    // Player 1 scores: 0 -> 15
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.player1.points, .fifteen, "Player 1 should have 15")
    
    // Player 1 scores again: 15 -> 30
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.player1.points, .thirty, "Player 1 should have 30")
    
    // Player 1 scores again: 30 -> 40
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.player1.points, .forty, "Player 1 should have 40")
}

func testSimpleWin() {
    print("\n🏸 Testing Simple Win...")
    var game = Game()
    
    // Player 1 gets 40, Player 2 has 0
    game = Game(player1: Player(points: .forty), player2: Player(points: .love))
    
    // Player 1 scores the winning point
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.state, .won, "Game should be won")
}

func testDeuce() {
    print("\n🏸 Testing Deuce...")
    var game = Game()
    
    // Both players get to 40
    game = Game(player1: Player(points: .forty), player2: Player(points: .forty))
    ScoreDisplay.show(game)
    checkEqual(game.state, .deuce, "Should be deuce when both have 40")
}

func testAdvantage() {
    print("\n🏸 Testing Advantage...")
    var game = Game()
    
    // Start at deuce
    game = Game(state: .deuce, player1: Player(points: .forty), player2: Player(points: .forty))
    
    // Player 1 scores from deuce
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.player1.points, .advantage, "Player 1 should have advantage")
}

func testWinFromAdvantage() {
    print("\n🏸 Testing Win from Advantage...")
    var game = Game()
    
    // Player 1 has advantage, Player 2 has 40
    game = Game(player1: Player(points: .advantage), player2: Player(points: .forty))
    
    // Player 1 scores the winning point
    game = TennisScorer.awardPoint(to: 1, game: game)
    ScoreDisplay.show(game)
    checkEqual(game.state, .won, "Player 1 should win the game")
}

// 🏃‍♂️ Run all the simple tests
print("TENNIS GAME TESTS")
print("===================")

testBasicScoring()
testSimpleWin()
testDeuce()
testAdvantage()
testWinFromAdvantage()

print("\n All tests done!")

// 📋 Simple test helper
func checkEqual<T: Equatable>(_ actual: T, _ expected: T, _ message: String = "") {
    if actual == expected {
        print("   ✅ \(message)")
    } else {
        print("   ❌ \(message)")
        print("      Expected: \(expected), Got: \(actual)")
    }
}
