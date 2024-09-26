use std::fmt;
use Piece::*;
pub mod position;
use position::*;
pub mod moveset;
pub mod board;
use board::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    King(Color),
    Queen(Color),
    Bishop(Color),
    Rook(Color),
}

impl Piece {
    /// Returns a char based on the color and class of the piece
    pub fn char(&self) -> char {
        /// Makes the char uppercase if white, lowercase if black, if it errors. f will be returned
        fn color_case(char: char, color: &Color) -> char {
            match color {
                Color::Black => char.to_lowercase().next().unwrap_or('f'),
                Color::White => char.to_uppercase().next().unwrap_or('F'),
            }
        }

        // matches the colored piece to the right char 
        match self {
            Pawn(color) => color_case('p', color),
            Knight(color) => color_case('n', color),
            King(color) => color_case('k', color),
            Queen(color) => color_case('q', color),
            Bishop(color) => color_case('b', color),
            Rook(color) => color_case('r', color),
        }
    }
    //Copilot wrote this :)
    pub fn get_color(&self) -> Color {
        match self {
            Pawn(color) | Knight(color) | King(color) | Queen(color) | Bishop(color) | Rook(color) => *color,
        }
    }
}

#[derive(Debug)]
pub enum ChessError {
    IllegalMove,
    GameAlreadyOver,
    IllegalSpawn,
    NoPiece,
    OutOfBounds,
    PromotionError(String),
}


///Game
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    pub turn: Color,
    pub board: Board,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game::empty();
        game.init();
        game
    }

    /// Creates new game with an empty board
    pub fn empty() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            turn: Color::White,
            board: Board::new(),
        }
    }

    //TODO: Do proper error handling
    pub fn init(&mut self) {
        // Vita pjäser
        //TODO: Handle results
        let color = Color::Black;
        let mut position = Position::new(0, 7).unwrap();
        let piece_array = [
            Rook(color),
            Knight(color),
            Bishop(color),
            Queen(color),
            King(color),
            Bishop(color),
            Knight(color),
            Rook(color),
        ];

        // Vita coola grabbar
        for p in piece_array {
            self.board.spawn_piece(p,&position).expect("Could not spawn piece");
            position.x += 1;
        }
        // Flytta ned ett steg
        position = Position::new(0, 6).unwrap();

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn(color), &position).expect("Could not spawn piece");
            position.x += 1
        }

        let color = Color::White;
        position = Position::new(0, 0).unwrap();
        let piece_array = [
            Rook(color),
            Knight(color),
            Bishop(color),
            Queen(color),
            King(color),
            Bishop(color),
            Knight(color),
            Rook(color)
        ];

        // Svarta coolingar
        for p in piece_array {
            self.board.spawn_piece(p, &position).expect("Could not spawn piece");
            position.x += 1
        }

        // Flytta upp ett steg
        position = Position::new(0, 1).unwrap();

        // svarta bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn(color), &position).expect("Could not spawn piece");
            position.x += 1
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) -> Result<GameState, ChessError> {
        if self.state == GameState::GameOver {
            return Err(ChessError::GameAlreadyOver);
        }
        if self.board.get_piece(&from.into()).is_none() {
            return Err(ChessError::NoPiece);
        }

        //Get the possible moves for the piece
        if let Some(possible_moves) = self.get_possible_moves(from) {
            //Check if the move is in the possible moves
            if possible_moves.contains(to) {

                //Check so that the move does not put the player in check
                if self.is_check(to){
                    return Err(ChessError::IllegalMove);
                }

                //Move the piece & overwrite the piece in the new position
                if let Some(piece) = self.board.get_piece(&from.into()) {
                    self.board.set_piece(piece, &to.into());
                }
                //Remove the piece from the old position
                self.board.despawn_piece(&from.into());

                //Change the turn
                self.turn = self.turn.other();
                //Check for check of the next player
                let king_board_position = &self.board.get_king_position(self.turn).unwrap().into();
                if self.is_check(king_board_position) {
                    self.state = GameState::Check;
                }else {
                    self.state = GameState::InProgress;
                }

                return Ok(self.state);
            } else {
                return Err(ChessError::IllegalMove);
            }
        }

        Err(ChessError::IllegalMove)
    }

    /// Promote
    pub fn promote_pawn(&mut self, pawn_position: &BoardPosition, promoted_piece: Piece) -> Result<(), ChessError> {
        //Get the piece at the position
        if let Some(pawn) = self.board.get_piece(&pawn_position.into()) {
            //Check if it is a pawn
            if let Pawn(_) = pawn {
                //Check if the new piece is a king or a pawn
                if let King(_) = promoted_piece {
                    return Err(ChessError::PromotionError("Cannot promote to king".to_string()));
                }
                if let Pawn(_) = promoted_piece {
                    return Err(ChessError::PromotionError("Cannot promote to pawn".to_string()));
                }
                //Change the piece
                self.board.set_piece(promoted_piece, &pawn_position.into());
            }
        }
        return Ok(());
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, position: &BoardPosition) -> Option<Vec<BoardPosition>> {
        //It the position has a piece
        let pos = &position.into();
        if let Some(piece) =  self.board.get_piece(pos) {
            //Get the moveset for that piece
            let moveset = moveset::get_moveset(piece);

            let mut legal_moves: Vec<BoardPosition> = vec![];
            for move_action in moveset.moves.into_iter() {

            'step: for step in 1..=moveset.steps {

                //Check to see if there is a piece on this place
                let next_position = move_action.get_position(pos, step);
                if let Ok(next_step) = next_position {
                    if let Some(p) = self.board.get_piece(&next_step) {
                    if p.get_color() == piece.get_color() {
                            break 'step;
                        } else {
                            //push then break
                            legal_moves.push(next_step.into());
                            break 'step;
                        }
                    } else {
                        //Push this step
                        legal_moves.push(next_step.into());
                    }
                }else{
                    break 'step;
                }
            }
           }
           return Some(legal_moves);
        }
        None
    }

    /// Check if the current player is in check based on the given position.
    pub fn is_check(&self, position: &BoardPosition) -> bool {
        //Check all the possible moves for all other pieces from the given position.

        //Get all pieces of the other color and dedup them so we only check each pieces moves once
        //This means we wont have to check the color later
        let mut pieces_to_check = self.board.get_all_pieces_of_color(self.turn.other());
        pieces_to_check.dedup();
        //For each piece, check if the piece of that type is in the possible moves
        for piece in pieces_to_check {
            
            let moves_for_piece = moveset::get_moveset(*piece);

            for action in moves_for_piece.moves {
                for step in 1..=moves_for_piece.steps {
                    let next_position = action.get_position(&position.into(), step);
                    
                    //If there is a piece of the right variant in the possible moves, return true
                    if let Ok(next_step) = next_position {
                        //Get the piece on this step
                        if let Some(p) = self.board.get_piece(&next_step) { //HELP should this be reference instead or will it not work cuz they are pointing to different blocks of memory?
                            if p == *piece {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_y, rank) in self.board.piece_array.iter().enumerate() {
            write!(f, "\n")?;
            for (_x, p) in rank.iter().enumerate() {
                if let Some(piece) = p {
                    write!(f, " {}", piece.char())?;
                } else {
                    write!(f, " *")?;
                }
            }
        }
        write!(f, "\n")
    }
}