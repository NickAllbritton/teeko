#[derive(Clone, Copy, PartialEq)]
pub enum BoardPiece {
    None,
    Red,
    Black
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5] {
    return [[BoardPiece::None; 5]; 5];
}

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5],
    pub current_player: BoardPiece,
    pub pieces_dropped: [i32;2]
}

impl GameState {

    pub fn jumbl_board(&mut self) {
        // TODO: Actually jumble the board lol
        self.board[1][0] = BoardPiece::Red;
        self.board[2][0] = BoardPiece::Black;
        self.board[4][4] = BoardPiece::Red;
        self.board[3][2] = BoardPiece::Black;
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        if row > 4 || col > 4 {
            return;
        }
        if self.pieces_dropped[self.index_of_piece(self.current_player)] >= 4 {
            return;
        }
        // First move cannot be the center of the board
        if self.pieces_dropped[self.index_of_piece(BoardPiece::Black)] == 0 && row == 2 && col == 2 {
            return;
        }
        if self.board[row][col] != BoardPiece::None {
            return;
        }

        self.board[row][col] = self.current_player;
        self.next_player();
    }

    fn next_player(&mut self) {
        self.pieces_dropped[self.index_of_piece(self.current_player)] += 1; // Add a piece to the
                                                                            // tracker variable
        match self.current_player {
            BoardPiece::Red => { self.current_player = BoardPiece::Black }
            BoardPiece::Black => { self.current_player = BoardPiece::Red }
            _ => {}
        }
    }

    fn index_of_piece(&self, piece: BoardPiece) -> usize {
        if piece == BoardPiece::Black {
            return 0;
        }
        return 1;
    }


}
