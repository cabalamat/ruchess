/* board.rs = a chess board/position */

// a value of a square on the chess board:
enum SqValue {
    OffBoard,               // off-board sentinel
    Empty,                  // empty square
    WP, WN, WB, WR, WQ, WK, // white pieces
    BP, BN, BB, BR, BQ, BK, // black pieces
}


/*end*/
