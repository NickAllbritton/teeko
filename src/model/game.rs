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
    pub pieces_dropped: [i32;2],
    pub phase2: bool,
    history: Vec<PieceDropCommand>,
    history_pos: usize
}

impl GameState {

    pub fn new() -> Self {
        Self {
            board: make_blank_board(),
            current_player: BoardPiece::Black, // Black moves First
            pieces_dropped: [0, 0],
            phase2: false, // Begin in phase 1 not phase 2 because 2 is after 1
            history: Vec::new(),
            history_pos: 0
        }
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        let command = PieceDropCommand {
            row: row,
            col: col,
            player: self.current_player
        }; 
        if !command.is_valid(self) {
            println!("Invalid move!");
            return;
        }
        if self.history.len() > 0 {
            let elements_to_remove = self.history.len() - (self.history_pos + 1);
            for _ in 0..elements_to_remove {
                self.history.pop();
            }
        }
        command.perform(self);
        self.history.push(command);
        self.history_pos = self.history.len() - 1;
    }

    pub fn redo_action(&mut self) {
        if self.history_pos + 1 >= self.history.len() {
            return;
        }

        self.history_pos += 1;

        let command: PieceDropCommand = self.history[self.history_pos].copy();
        command.perform(self);
    }

    pub fn undo_action(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        let command: PieceDropCommand = self.history[self.history_pos].copy();

        if self.history.len() != 0 {
            command.undo(self);
            if self.history_pos > 0 {
                self.history_pos -= 1;
                self.change_phase_if_nec();
            }
        }
    }

    pub fn change_phase_if_nec(&mut self)
    {
        // if at least 8 pieces have been dropped -> go phase2
        self.phase2 = self.pieces_dropped[0] + self.pieces_dropped[1] == 8 ||
            (self.pieces_dropped[0] + self.pieces_dropped[1] == 7 && self.history[self.history_pos].player != self.current_player); 
    }

    fn next_player(&mut self) {
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

struct PieceDropCommand {
    pub row: usize,
    pub col: usize,
    pub player: BoardPiece
}

impl PieceDropCommand {

    pub fn perform(&self, game: &mut GameState) {
        if game.phase2 && game.pieces_dropped[0]+game.pieces_dropped[1]==8{
            self.undo(game);
            // Do not switch players!
            // Do not change phase!
            println!("Perform piece remove");
        }
        else {
            game.board[self.row][self.col] = game.current_player;
            game.pieces_dropped[game.index_of_piece(game.current_player)] += 1; // Add a piece to the
                                                                                // tracker variable
            game.next_player();
            game.change_phase_if_nec();
            println!("Perform piece placement");
        }
    }

    pub fn undo(&self, game: &mut GameState) {
        if game.pieces_dropped[game.index_of_piece(self.player)] == 0 {
            return;
        }

        game.pieces_dropped[game.index_of_piece(game.current_player)] -= 1; // Removed a piece
        game.board[self.row][self.col] = BoardPiece::None;
        // Do not switch players
        game.current_player = self.player;
    }

    pub fn is_valid(&self, game: & GameState) -> bool {
        println!("Selected ({}, {})", self.row, self.col);
        if self.row > 4 || self.col > 4 {
            return false;
        }
        if game.phase2 {
            println!("Phase 2");
            // Phase 2
            // if it is in phase 2 but does not have 4 pieces placed, that means the player has
            // removed a piece
            if game.pieces_dropped[game.index_of_piece(self.player)] < 4 {
                if game.board[self.row][self.col] == BoardPiece::None {
                    // return false if the piece is more than 1 piece away
                    let distancex: isize = isize::abs(<usize as TryInto<isize>>::try_into(game.history[game.history_pos].row).unwrap() - <usize as TryInto<isize>>::try_into(self.row).unwrap());
                    let distancey: isize = isize::abs(<usize as TryInto<isize>>::try_into(game.history[game.history_pos].col).unwrap() - <usize as TryInto<isize>>::try_into(self.col).unwrap());
                    println!("Distance in the x: {}", distancex);
                    println!("Distance in the y: {}", distancey);
                    let within_one_move: bool = distancex <= 1 && distancey <= 1;
                    println!("The selected spot is less than 2 moves away: {}", within_one_move);
                    return within_one_move;
                }
                else {
                    // If the player has removed a piece they cannot place one on any non-empty
                    // place
                    println!("Phase 2 but the selected position is not empty");
                    return false;
                }
            }
            else {
                // if all 4 of your pieces are down in phase 2 you need to remove a piece
                return game.board[self.row][self.col] == self.player; // are your removing your own
                                                                      // piece?
            }
        }
        else {
            println!("Phase 1");
            // Phase 1
            // If black has not placed any pieces, it is first move. First move cannot be the
            // center 
            if game.pieces_dropped[game.index_of_piece(BoardPiece::Black)] == 0 {
                return self.row != 2 || self.col !=2;
            }
        }
        return true;
    }

    pub fn copy(&self) -> Self {
        Self { 
            row: self.row,
            col: self.col,
            player: self.player
        }
    }

}





































