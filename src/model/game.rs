#[derive(Clone, Copy)]
pub enum BoardPiece {
    None,
    Red,
    Black
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5]; {
    return [[BoardPiece::None; 5]; 5];
}

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5]
}

impl GameState {

    pub fn jumbl_board(&mut self) {
        // TODO: Actually jumble the board lol
        self.board[1][0] = BoardPiece::Red;
        self.board[2][0] = BoardPiece::Black;
    }

}
