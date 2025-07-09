def HelloWorld():
    print("Hello, World!")

class Game():
    # score = "love-all"
    score_list = ["love", "15", "30", "40"]
    _playerOneScore = 0
    _playerTwoScore = 0
    def PrintScore(self):
        # if self._playerOneScore == 3 and self._playerTwoScore == 3:
        #     return "deuce"
        
        if self._playerOneScore >= 3 and self._playerTwoScore >= 3 and self._playerOneScore == self._playerTwoScore:
            return "deuce"
        
        if self._playerOneScore == self._playerTwoScore:
            return f"{self.score_list[self._playerOneScore]}-all"
        
        if self._playerOneScore > 3:
            if self._playerTwoScore == self._playerOneScore - 1:
                return "advantage PlayerOne"
            return "PlayerOne wins"

        if self._playerTwoScore > 3:
            if self._playerOneScore == self._playerTwoScore - 1:
                return "advantage PlayerTwo"
            return "PlayerTwo wins"
        
        # if self._playerOneScore > 3: 
        #     return "PlayerOne wins"
        
        # if self._playerTwoScore > 3:
        #     return "PlayerTwo wins"

        return f"{self.score_list[self._playerOneScore]}-{self.score_list[self._playerTwoScore]}"
    def _playerOneScores(self):
        self._playerOneScore += 1
    
    def _playerTwoScores(self):
        self._playerTwoScore += 1



# main
HelloWorld()