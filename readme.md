For a certain board state, check whether a play is valid.

Board has a state
Play has coords and status = unchecked

board.check(play) -> play_result

board.play(play_result) -> void

alt.

board.play(play_result) -> Result

Goal is:
- check whether the play is valid && return the number of points before doing the play
- do the play whithout rechecking whether it's invalid

How to calculate board state? One way or two way?
Two way would be current_player + coordinates with W + coordinates with B

An alternative way is to...
- as you check, you modify the board
- if the check becomes invalid, you throw away the board (but what if it's valid on the first direction but invalid on the second)?

alt.

- store the next possible play inside the board
- that's not good if you're trying to determine what is the best to play (unless of course you only persisted the best play if the score was higher)

game.check(&board, coord) -> PlayResult
game.check_if_better(&board, coord) -> PlayResult

game.commit_last_play(&board) -> Result

enum PlayResult {
    ValidWithScore(ValidPlay),
    Invalid
} 

struct ValidPlay {
    score: usize,
    coord: (usize, usize)
}