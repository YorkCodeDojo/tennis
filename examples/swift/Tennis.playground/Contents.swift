import UIKit

enum GameState {
    case warmup
    case playing
    case deuce
    case won
    case lost
}

enum Points {
    case love
    case fifteen
    case thirty
    case forty
    case avantage
    case won
}

struct Player {
    let points : Points
    
    init(points: Points = .love) {
        self.points = points
    }
}

struct Game {
    
    let state : GameState
    
    let players : Players
    
    init(state: GameState = .warmup, players: Players = Players()) {
        self.state = state
        self.players = players
    }
    
}

enum PlayerType {
    case server
    case reciever
}

struct Players {
    
    let server: Player
    let reciever: Player
    
    init(server: Player = Player(), reciever: Player = Player()) {
        self.server = server
        self.reciever = reciever
    }
    
}

struct Umpire {
    
    static func awardPoints(to player: PlayerType, game: Game) -> Game {
        
        // Handle special case: if opponent has advantage, reset to deuce
        if game.state == .playing {
            if player == .server && game.players.reciever.points == .avantage {
                let resetPlayers = Players(
                    server: Player(points: .forty),
                    reciever: Player(points: .forty)
                )
                return Game(state: .deuce, players: resetPlayers)
            } else if player == .reciever && game.players.server.points == .avantage {
                let resetPlayers = Players(
                    server: Player(points: .forty),
                    reciever: Player(points: .forty)
                )
                return Game(state: .deuce, players: resetPlayers)
            }
        }
        
        // First, increment the player's points
        let updatedPlayers = incrementPlayerPoints(player: player, players: game.players)
        
        // Check if game is won after awarding points
        let isGameWon = byAwardingPlayerPointIsGameWon(to: player, game: Game(state: game.state, players: updatedPlayers))
        
        if(isGameWon){
            
            print("Game won by (\(player))")
            return Game(state: .won, players: updatedPlayers)
        }
        
        // Check if game should be in deuce state
        let newState = determineGameState(players: updatedPlayers, currentState: game.state)
        
        return Game(state: newState, players: updatedPlayers)
        
    }
    
    static func incrementPlayerPoints(player: PlayerType, players: Players) -> Players {
        if player == .server {
            let newServerPoints = nextPoint(from: players.server.points)
            let updatedServer = Player(points: newServerPoints)
            return Players(server: updatedServer, reciever: players.reciever)
        } else {
            let newReceiverPoints = nextPoint(from: players.reciever.points)
            let updatedReceiver = Player(points: newReceiverPoints)
            return Players(server: players.server, reciever: updatedReceiver)
        }
    }
    
    static func nextPoint(from currentPoints: Points) -> Points {
        switch currentPoints {
        case .love:
            return .fifteen
        case .fifteen:
            return .thirty
        case .thirty:
            return .forty
        case .forty:
            return .avantage
        case .avantage:
            return .won
        case .won:
            return .won
        }
    }
    
    static func determineGameState(players: Players, currentState: GameState) -> GameState {
        let serverPoints = players.server.points
        let receiverPoints = players.reciever.points
        
        // Check for deuce condition (both at forty)
        if serverPoints == .forty && receiverPoints == .forty {
            return .deuce
        }
        
        // If either player has advantage, the game is in playing state
        if serverPoints == .avantage || receiverPoints == .avantage {
            return .playing
        }
        
        return .playing
    }
    
    static func byAwardingPlayerPointIsGameWon(to player: PlayerType, game: Game) -> Bool {
        
        let serverPoints = game.players.server.points
        let receiverPoints = game.players.reciever.points
        
        if player == .server {
            // Server wins if:
            // 1. They have advantage and receiver doesn't have forty (advantage -> win)
            // 2. They have forty and receiver has less than forty (straight win)
            // 3. They would get .won points (meaning they had advantage and got another point)
            return (serverPoints == .avantage && receiverPoints != .forty) ||
                   (serverPoints == .forty && receiverPoints != .forty && receiverPoints != .avantage) ||
                   (serverPoints == .won)
        } else {
            // Receiver wins if:
            // 1. They have advantage and server doesn't have forty (advantage -> win)
            // 2. They have forty and server has less than forty (straight win)
            // 3. They would get .won points (meaning they had advantage and got another point)
            return (receiverPoints == .avantage && serverPoints != .forty) ||
                   (receiverPoints == .forty && serverPoints != .forty && serverPoints != .avantage) ||
                   (receiverPoints == .won)
        }
        
    }
    
}


struct Scoreboard {
    
    static func printGame(game: Game) {
        
        print("Game state: \(game.state)")
        print("Player 1: \(game.players.server.points). Player 2: \(game.players.reciever.points).")
        
    }
}


// MARK: - Playground Tests
func testInitialGameState() {
    print("\n🧪 Testing Initial Game State...")
    let game = Game()
    
    assertEqual(game.state, .warmup, "Game should start in warmup state")
    assertEqual(game.players.server.points, .love, "Server should start with love")
    assertEqual(game.players.reciever.points, .love, "Receiver should start with love")
}

func testPointProgression() {
    print("\n🧪 Testing Point Progression...")
    var game = Game()
    
    // Award first point to server
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.players.server.points, .fifteen, "Server should have fifteen after first point")
    assertEqual(game.players.reciever.points, .love, "Receiver should still have love")
    assertEqual(game.state, .playing, "Game should be in playing state")
    
    // Award second point to server
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.players.server.points, .thirty, "Server should have thirty after second point")
    
    // Award third point to server
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.players.server.points, .forty, "Server should have forty after third point")
}

func testServerWinsGame() {
    print("\n🧪 Testing Server Wins Game...")
    var game = Game()
    
    // Server wins 4-0
    game = Umpire.awardPoints(to: .server, game: game) // 15-0
    game = Umpire.awardPoints(to: .server, game: game) // 30-0
    game = Umpire.awardPoints(to: .server, game: game) // 40-0
    game = Umpire.awardPoints(to: .server, game: game) // Game won
    
    assertEqual(game.state, .won, "Game should be won")
}

func testDeuceScenario() {
    print("\n🧪 Testing Deuce Scenario...")
    var game = Game()
    
    // Both players reach 40 (deuce)
    game = Umpire.awardPoints(to: .server, game: game)    // 15-0
    game = Umpire.awardPoints(to: .server, game: game)    // 30-0
    game = Umpire.awardPoints(to: .server, game: game)    // 40-0
    
    game = Umpire.awardPoints(to: .reciever, game: game)  // 40-15
    game = Umpire.awardPoints(to: .reciever, game: game)  // 40-30
    game = Umpire.awardPoints(to: .reciever, game: game)  // 40-40 (deuce)
    
    assertEqual(game.state, .deuce, "Game should be in deuce state")
    assertEqual(game.players.server.points, .forty, "Server should have forty")
    assertEqual(game.players.reciever.points, .forty, "Receiver should have forty")
}

func testAdvantageAndWin() {
    print("\n🧪 Testing Advantage and Win...")
    var game = Game()
    
    // Set up deuce scenario
    game = Game(state: .deuce, players: Players(
        server: Player(points: .forty),
        reciever: Player(points: .forty)
    ))
    
    // Server gets advantage
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.players.server.points, .avantage, "Server should have advantage")
    assertEqual(game.players.reciever.points, .forty, "Receiver should still have forty")
    assertEqual(game.state, .playing, "Game should be in playing state")
    
    // Server wins the game
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.state, .won, "Game should be won")
}

func testCompleteDeuceBattle() {
    print("\n🧪 Testing Complete Deuce Battle...")
    var game = Game()
    
    // Set up deuce
    game = Game(state: .deuce, players: Players(
        server: Player(points: .forty),
        reciever: Player(points: .forty)
    ))
    
    // Server gets advantage
    game = Umpire.awardPoints(to: .server, game: game)
    assertEqual(game.state, .playing, "Game should be playing after advantage")
    assertEqual(game.players.server.points, .avantage, "Server should have advantage")
    
    // Receiver equalizes - back to deuce
    game = Umpire.awardPoints(to: .reciever, game: game)
    assertEqual(game.state, .deuce, "Game should be back to deuce")
    assertEqual(game.players.server.points, .forty, "Server should be back to forty")
    assertEqual(game.players.reciever.points, .forty, "Receiver should be back to forty")
    
    // Receiver gets advantage this time
    game = Umpire.awardPoints(to: .reciever, game: game)
    assertEqual(game.state, .playing, "Game should be playing with receiver advantage")
    assertEqual(game.players.reciever.points, .avantage, "Receiver should have advantage")
    
    // Receiver wins the game
    game = Umpire.awardPoints(to: .reciever, game: game)
    assertEqual(game.state, .won, "Game should be won by receiver")
}

func testAdvantageBackToDeuce() {
    print("\n🧪 Testing Advantage Back to Deuce...")
    var game = Game()
    
    // Set up advantage scenario - server has advantage
    game = Game(state: .playing, players: Players(
        server: Player(points: .avantage),
        reciever: Player(points: .forty)
    ))
    
    // Receiver wins point, should go back to deuce
    game = Umpire.awardPoints(to: .reciever, game: game)
    assertEqual(game.state, .deuce, "Game should be back to deuce")
    assertEqual(game.players.server.points, .forty, "Server should be back to forty")
    assertEqual(game.players.reciever.points, .forty, "Receiver should be back to forty")
}

func testReceiverWinsGame() {
    print("\n🧪 Testing Receiver Wins Game...")
    var game = Game()
    
    // Receiver wins 4-1
    game = Umpire.awardPoints(to: .server, game: game)    // 15-0
    game = Umpire.awardPoints(to: .reciever, game: game)  // 15-15
    game = Umpire.awardPoints(to: .reciever, game: game)  // 15-30
    game = Umpire.awardPoints(to: .reciever, game: game)  // 15-40
    game = Umpire.awardPoints(to: .reciever, game: game)  // Game won by receiver
    
    assertEqual(game.state, .won, "Game should be won by receiver")
}

// Run the tests
print("🏸 Running Tennis Game Tests...")

testInitialGameState()
testPointProgression()
testServerWinsGame()
testDeuceScenario()
testAdvantageAndWin()
testCompleteDeuceBattle()
testAdvantageBackToDeuce()
testReceiverWinsGame()

print("\n🎾 All tests completed!")


// Simple test assertion functions for playground
func assertEqual<T: Equatable>(_ actual: T, _ expected: T, _ message: String = "") {
    if actual == expected {
        print("✅ PASS: \(message)")
    } else {
        print("❌ FAIL: \(message)")
        print("   Expected: \(expected)")
        print("   Actual: \(actual)")
    }
}

func assertTrue(_ condition: Bool, _ message: String = "") {
    if condition {
        print("✅ PASS: \(message)")
    } else {
        print("❌ FAIL: \(message)")
    }
}
