/* main.rs */

// a value of a square on the chess board:

#[derive(Debug)]
enum SqValue {
    OffBoard,               // off-board sentinel
    Empty,                  // empty square
    WP, WN, WB, WR, WQ, WK, // white pieces
    BP, BN, BB, BR, BQ, BK, // black pieces
}

impl SqValue {
    fn to_string(self: SqValue) -> &'static str {
        match self {
            SqValue::OffBoard => "-",
            SqValue::Empty => " ",
            SqValue::WP => "p",
            SqValue::WN => "n",
            SqValue::WB => "b",
            SqValue::WR => "r",
            SqValue::WQ => "q",
            SqValue::WK => "k",
            SqValue::BP => "P",
            SqValue::BN => "N",
            SqValue::BB => "B",
            SqValue::BR => "R",
            SqValue::BQ => "Q",
            SqValue::BK => "K",
        }
    }
}

// whose move is it?
#[derive(Debug)]
enum Mover {
    White,
    Black
}    

/* a chess position, including the location of the pieces, whose turn it
is, castling information, etc 
*/
struct Board {
    sq: [SqValue; 64],
    w_castle_k: bool, // can white castle kingside? 
    w_castle_q: bool, // can white castle queenside? 
    b_castle_k: bool, // can black castle kingside? 
    b_castle_q: bool, // can black castle queenside? 
    mover: Mover,
}

impl Board {
    fn empty() -> Board {
        /* return an empty board */
        Board {
            sq: [SqValue::Empty, SqValue::Empty, 
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty,
                 SqValue::Empty, SqValue::Empty],
            w_castle_k: true,
            w_castle_q: true,
            b_castle_k: true, 
            b_castle_q: true,
            mover: Mover::White,
        }
    }
    
    fn sqix(rank: i32, file: i32) -> usize {
        /* convert from rank and file coordinates to 
        sq coordinates */
        (file + rank*8 - 9) as usize
    }
    
    
    fn to_string(self: &Board) -> String {
        let mut s = "*** the board ***\n".to_string();
        for file in 1..8 {
            for rank in (1..8).rev() {
                let si = Board::sqix(rank, file);
                let sqv = &self.sq[si];
                s += sqv.to_string();
                s += " ";
                
            }//for rank
        }//for file
        s
    }
    
}

fn main() {
    println!("Hello, world!");
    
    let sv = SqValue::WK;
    println!("sv is {}\n", sv.to_string());
    
    let b = Board::empty();
    println!("board b:\n{}", b.to_string());
}

/*end*/
