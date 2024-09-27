use std::fmt;
use Piece::*;
pub mod position;
use position::*;
pub mod board;
pub mod moveset;
use board::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    ///Game is in progress, the next player can make a move
    InProgress,
    ///Game is in check, the next player must make a move to get out of check
    Check,
    ///Game is over, the color is the winner
    GameOver(Color),
    ///Game is in promotion, the next player must promote a pawn to proceed into another game state
    Promotion(BoardPosition),
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the other color
    fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
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
    /// Uppercase if white, lowercase if black
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

    ///Returns the color of the piece
    pub fn get_color(&self) -> Color {
        match self {
            Pawn(color) | Knight(color) | King(color) | Queen(color) | Bishop(color)
            | Rook(color) => *color,
        }
    }
}

#[derive(Debug)]
pub enum ChessError {
    /// Occurs when trying to move a piece to an illegal position
    IllegalMove,
    /// Occurs when trying to move a piece when the game is over
    GameAlreadyOver,
    /// Occurs when trying to spawn a piece on top of another piece
    IllegalSpawn,
    /// Occurs when there is no piece at the given position
    NoPiece,
    /// Occurs when trying to access a position outside the board
    OutOfBounds,
    /// Occurs when trying to promote to an illegal piece
    PromotionError(String),
}

/// Game
pub struct Game {
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

    /// Initialises the game by placing all pieces on the board in the default chess starting positions.
    pub fn init(&mut self) {
        // Vita pjäser
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
            self.board
                .spawn_piece(p, &position)
                .expect("Could not spawn piece");
            position.x += 1;
        }
        // Flytta ned ett steg
        position = Position::new(0, 6).unwrap();

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board
                .spawn_piece(Pawn(color), &position)
                .expect("Could not spawn piece");
            position.x += 1
        }

        // Svarta pjäser
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
            Rook(color),
        ];

        // Svarta coolingar
        for p in piece_array {
            self.board
                .spawn_piece(p, &position)
                .expect("Could not spawn piece");
            position.x += 1
        }

        // Flytta upp ett steg
        position = Position::new(0, 1).unwrap();

        // svarta bondlurkar
        for _ in 1..=8 {
            self.board
                .spawn_piece(Pawn(color), &position)
                .expect("Could not spawn piece");
            position.x += 1
        }
    }

    /// Moves the piece from the `from` position to the `to` position on the board.
    /// Returns the new game state or a chess error
    pub fn move_piece(
        &mut self,
        from: &BoardPosition,
        to: &BoardPosition,
    ) -> Result<GameState, ChessError> {
        if let GameState::GameOver(_) = self.state {
            return Err(ChessError::GameAlreadyOver);
        }
        if self.board.get_piece(&from.into()).is_none() {
            return Err(ChessError::NoPiece);
        }
        if let GameState::Promotion(_) = self.state {
            return Err(ChessError::PromotionError("Must promote pawn".to_string()));
        }

        //Get the possible moves for the piece
        if let Some(possible_moves) = self.get_possible_moves(from) {
            //Check if the move is in the possible moves
            if possible_moves.contains(to) {
                //Move the piece
                self.board.move_piece(&from.into(), &to.into());
                //Change the turn
                self.turn = self.turn.other();
                //Evaluate the game state
                self.evaluate_game_state();

                return Ok(self.state);
            } else {
                return Err(ChessError::IllegalMove);
            }
        }

        Err(ChessError::IllegalMove)
    }

    /// Updates the internal game state by checking for 
    /// * Win
    /// * Promotion
    /// * Check
    
    fn evaluate_game_state(&mut self) -> GameState {
        //Check for win
        if self.board.black_king_position.is_none() {
            self.state = GameState::GameOver(Color::White);
            return self.state;
        }else if self.board.white_king_position.is_none() {
            self.state = GameState::GameOver(Color::Black);
            return self.state;
        }

        //Check for promotion
        let rank_eight = self.board.get_rank(Rank::Eight);
        let rank_one = self.board.get_rank(Rank::One);

        //function that returns the position of the first pawn that can be promoted
        fn get_promotion_position(rank_array: &[Option<Piece>], rank: Rank) -> Option<BoardPosition> {
            let can_promote = |piece: Piece| {
                match piece {
                    Pawn(Color::White) => true,
                    Pawn(Color::Black) => true,
                    _ => false,
                }
            };

            for (i, p) in rank_array.iter().enumerate() {
                if let Some(p) = p {
                    if can_promote(*p) {
                        return Some(BoardPosition::new(File::from(i), rank));
                    }
                }
            }
            None
        }

        if let Some(promotion_position) = get_promotion_position(&rank_eight, Rank::Eight) {
            self.state = GameState::Promotion(promotion_position);
            return self.state;
        } else if let Some(promotion_position) = get_promotion_position(&rank_one, Rank::One) {
            self.state = GameState::Promotion(promotion_position);
            return self.state;
        }

        //Check for check
        self.state = if self.is_check(&self.board.white_king_position.unwrap().into()) {
            GameState::Check
        } else if self.is_check(&self.board.black_king_position.unwrap().into()) {
            GameState::Check
        } else {
            self.state
        };
        self.state
    }
    /// Promotes the pawn at the game state promotions position to the new piece.
    pub fn promote_pawn(&mut self, new_piece: Piece) -> Result<GameState, ChessError> {
        //Checks to see if the game state is promotion.
        if let GameState::Promotion(pawn_position) = self.state {
            // Make sure the new piece is a legal promotion piece
            if let Pawn(_) = new_piece {
                return Err(ChessError::PromotionError(format!(
                    "Cannot promote pawn at {:?} to {:?}",
                    pawn_position, new_piece
                )));
            }
            self.board.set_piece(new_piece, &pawn_position.into());
            self.evaluate_game_state();
            return Ok(self.state);
        } else {
            return Err(ChessError::PromotionError("No pawn to promote".to_string()));
        }
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// Returns the possible moves for the piece at the given position.
    /// If there is no piece at the given position, None is returned.
    pub fn get_possible_moves(&mut self, position: &BoardPosition) -> Option<Vec<BoardPosition>> {
        //Get the piece at the given position
        if let Some(piece) = self.board.get_piece(&position.into()) {
            //Get the moveset for that piece variant
            let moveset = moveset::get_moveset(piece);
            let mut legal_moves: Vec<BoardPosition> = vec![];

            for move_action in moveset.moves.into_iter() {
                'step: for step in 1..=moveset.steps {
                    //Check to see if there is a piece on this place
                    let next_position = move_action.get_position(&position.into(), step);

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
                    } else {
                        break 'step;
                    }
                }
            }
            //Remove the current piece from the board
            let current_piece = self.board.get_piece(&position.into()).unwrap();
            self.board.despawn_piece(&position.into());
            //Keep all moves that does not put current player in check
            legal_moves.retain(|&x| !self.is_check(&x));
            //Put the piece back on the board
            self.board.set_piece(current_piece, &position.into());
            return Some(legal_moves);
        }
        None
    }

    /// Check if the current player is in check based on the given position.
    fn is_check(&self, position: &BoardPosition) -> bool {
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
                        if let Some(p) = self.board.get_piece(&next_step) {
                            //HELP should this be reference instead or will it not work cuz they are pointing to different blocks of memory?
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
