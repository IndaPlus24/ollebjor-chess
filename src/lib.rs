use std::fmt;
use Piece::*;
pub mod position;
use position::*;
pub mod board;
pub mod moveset;
use board::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
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

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq)]
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
    PromotionError,
    /// Occurs when trying to access a file that does not exist
    InvalidFile,
    /// Occurs when trying to access a rank that does not exist
    InvalidRank,
    /// Occurs when trying to access a position that does not exist
    InvalidPositionString,
    /// Occurs when a pawn needs to be promoted before the game can continue
    PromoteFirst,
    /// Occurs when trying to move a piece that is not the current players
    NotYourTurn
}

/// Game
pub struct Game {
    state: GameState,
    pub board: Board,
    moves_made: usize,
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
            board: Board::new(),
            moves_made: 0,
        }
    }

    /// Initialises the game by placing all pieces on the board in the default chess starting positions.
    pub fn init(&mut self) {
        // Vita pjäser
        let color = Color::Black;
        let mut position = Position::new(0, 7);
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
        position = Position::new(0, 6);

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board
                .spawn_piece(Pawn(color), &position)
                .expect("Could not spawn piece");
            position.x += 1
        }

        // Svarta pjäser
        let color = Color::White;
        position = Position::new(0, 0);
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
        position = Position::new(0, 1);

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
        self.pre_move_probe();
        if let GameState::GameOver(_) = self.get_game_state() {
            return Err(ChessError::GameAlreadyOver);
        }
        if let GameState::Promotion(_) = self.get_game_state() {
            return Err(ChessError::PromoteFirst);
        }
        if self.board.get_piece(&from.into()).is_none() {
            return Err(ChessError::NoPiece);
        }
        if let Some(piece) = self.get_piece(from) {
            if piece.get_color() != self.get_turn() {
                return Err(ChessError::NotYourTurn);
            }
        }

        //Get the possible moves for the piece
        if let Some(possible_moves) = self.get_possible_moves(from) {
            //Check if the move is in the possible moves
            if possible_moves.contains(to) {
                //Move the piece
                self.board.move_piece(&from.into(), &to.into());

                if self.is_promotion_available_at(to) {
                    self.state = GameState::Promotion(to.clone());
                    return Ok(self.state);
                }

                return Ok(self.post_move_probe());
            } else {
                return Err(ChessError::IllegalMove);
            }
        }

        Err(ChessError::IllegalMove)
    }

    fn is_won(&mut self) -> bool {
        //Check for win
        if self.board.black_king_position.is_none() || self.board.white_king_position.is_none() {
            return true;
        } else {
            return false;
        };
    }

    /// Checks wether or not a position is available for promotion
    fn is_promotion_available_at(&self, position: &BoardPosition) -> bool {
        //Check if the moved piece is a pawn and can be promoted
        if let Some(piece) = self.get_piece(position) {
            if let Pawn(_) = piece {
                match (position.rank, self.get_turn()) {
                    (Rank::Eight, Color::White) => {
                        return true;
                    }
                    (Rank::One, Color::Black) => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        return false;
    }

    /// Checks if the current player is in check
    fn is_check(&mut self) -> bool {
        //Get the position of the king of the current player
        let king_position = match self.get_turn() {
            Color::White => self.board.white_king_position.unwrap(),
            Color::Black => self.board.black_king_position.unwrap(),
        };

        let mut pieces_to_check = self.board.get_all_pieces_of_color(self.get_turn().other());
        pieces_to_check.dedup();

        let king_in_check_by = |piece: &Piece| {
            let moves_for_piece = moveset::get_moveset(*piece);

            for action in moves_for_piece.moves {
                'step: for step in 1..=moves_for_piece.steps {
                    let next_position = action.get_position(&king_position, step);

                    //if next_position is Ok, get the piece on that position
                    if let Some(next_step) = next_position {
                        //Get the piece on this step
                        if let Some(p) = self.board.get_piece(&next_step) {
                            if p == *piece && p.get_color() == self.get_turn().other() {
                                return true;
                            } else {
                                break 'step;
                            }
                        }
                    } else {
                        break 'step;
                    }
                }
            }
            return false;
        };

        if pieces_to_check.iter().any(|p| king_in_check_by(p)) {
            return true;
        } else {
            return false;
        }
    }

    /// Returns the color of the player who's turn it is
    pub fn get_turn(&self) -> Color {
        match self.moves_made % 2 {
            0 => Color::White,
            _ => Color::Black,
        }
    }

    fn post_move_probe(&mut self) -> GameState {
        //Change the turn
        self.moves_made += 1;

        //Check if the game is won
        if self.is_won() {
            self.state = GameState::GameOver(match self.board.white_king_position {
                Some(_) => Color::White,
                None => Color::Black,
            });
            return self.get_game_state();
        }

        //Check if the player put the other player in check
        if self.is_check() {
            self.state = GameState::Check;
            return self.get_game_state();
        }

        self.state = GameState::InProgress;
        return self.get_game_state();
    }

    fn pre_move_probe(&mut self) -> GameState {
        //Check if the game is won
        if self.is_won() {
            self.state = GameState::GameOver(match self.board.white_king_position {
                Some(_) => Color::White,
                None => Color::Black,
            });
            return self.get_game_state();
        }

        return self.get_game_state();
    }

    /// Promotes the pawn at the game state promotions position to the new piece.
    /// Returns the new game state or a chess error explaining why the promotion failed.
    pub fn promote_pawn(&mut self, new_piece: Piece) -> Result<GameState, ChessError> {
        //Checks to see if the game state is promotion
        if let GameState::Promotion(pawn_position) = self.state {
            // Make sure the new piece is a legal promotion piece
            match new_piece {
                Queen(_) | Rook(_) | Bishop(_) | Knight(_) => {}
                _ => {
                    return Err(ChessError::PromotionError);
                }
            }
            //Promote the pawn
            self.board.set_piece(new_piece, &pawn_position.into());
            self.post_move_probe();
            return Ok(self.get_game_state());
        } else {
            return Err(ChessError::PromotionError);
        }
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// Returns the possible moves for the piece at the given position.
    /// If there is no piece at the given position, None is returned. TODO: Make possible_moves not depend on the current turn of the game
    pub fn get_possible_moves(&mut self, position: &BoardPosition) -> Option<Vec<BoardPosition>> {
        //Get the piece at the given position
        if let Some(piece) = self.board.get_piece(&position.into()) {
            //Get the moveset for that piece variant
            let moveset = moveset::get_moveset(piece);
            let mut legal_moves: Vec<BoardPosition> = vec![];

            for move_action in moveset.moves {
                for pos in moveset::get_steps(&position.into(), &move_action, moveset.steps) {
                    if let Some(piece) = self.get_piece(&pos.try_into().unwrap()) {
                        if piece.get_color() == self.get_turn() {
                            break;
                        } else {
                            legal_moves.push(pos.try_into().unwrap());
                            break;
                        }
                    } else {
                        legal_moves.push(pos.try_into().unwrap());
                    }
                }
            }

            //START OF ERROR REGION
            //Remove the current piece from the board
            let current_piece = self.get_piece(position).unwrap();
            self.board.despawn_piece(&position.into());
            //Keep all moves that does not put current player in check
            legal_moves.retain(|&x| {
                // remove the piece from the new position
                let other_piece = self.board.take_piece(&x.into());
                //move the piece to the new position
                self.board.set_piece(current_piece, &x.into());
                //Check if the player is in check
                let keep = !self.is_check();
                //Remove the piece from the new position
                self.board.despawn_piece(&x.into());
                if other_piece.is_some() {
                    self.board.set_piece(other_piece.unwrap(), &x.into());
                }
                keep
            });
            //Put the piece back on the board
            self.board.set_piece(current_piece, &position.into());
            //END

            return Some(legal_moves);
        }
        None
    }

    /// Returns the piece as an option at the given position.
    pub fn get_piece(&self, position: &BoardPosition) -> Option<Piece> {
        self.board.get_piece(&position.into())
    }

    pub fn visualize_legal_moves(&mut self, position: &BoardPosition) {
        if let Some(piece) = self.get_piece(position) {
            println!("Legal moves for {:?} at position {:?}:", piece, position);
        } else {
            println!("No piece at position {:?}", position);
            return;
        }
        if let Some(moves) = self.get_possible_moves(position) {
            for (y, rank) in self.board.piece_array.iter().enumerate() {
                print!("\n");
                for (x, _file) in rank.iter().enumerate() {
                    let pos: BoardPosition =
                        Position::new(x, (7usize).abs_diff(y)).try_into().unwrap();
                    if moves.contains(&pos) {
                        print!(" {pos:?}");
                    } else {
                        print!("  *");
                    }
                }
            }
            print!("\n")
        }
        println!("\n--------------------------\n");
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
