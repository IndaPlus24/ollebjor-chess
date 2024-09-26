use std::{fmt};
use Piece::*;
pub mod position;
use position::*;
pub mod moveset;
use moveset::*;
pub mod board;
use board::*;

const MAX_STEPS: u8 = 100;

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
    IllegalSpawn,
    NoPiece,
    OutOfBounds,
}


///Game
pub struct Game {
    /* save board, active colour, ... */
    pub state: GameState,
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
            King(color),
            Queen(color),
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
            King(color),
            Queen(color),
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
        Err(ChessError::IllegalMove)
    }

    /// Set the piece type that a peasant becomes following a promotion.
    pub fn set_promotion(&mut self, piece: Piece) -> () {
        ()
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
                let next_step = move_action.get_position(pos, step);
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
            }
           }
           return Some(legal_moves);
        }
        None
    }
}


/// Implement print routine for Game.
///
/// Output example:
///(y)   
///    |:----------------------:|
/// 8  | R  Kn B  K  Q  B  Kn R |
/// 7  | P  P  P  P  P  P  P  P |
/// 6  | *  *  *  *  *  *  *  * |
/// 5  | *  *  *  *  *  *  *  * |
/// 4  | *  *  *  *  *  *  *  * |
/// 3  | *  *  *  *  *  *  *  * |
/// 2  | P  P  P  P  P  P  P  P |
/// 1  | R  Kn B  K  Q  B  Kn R |
///    |:----------------------:|
///      A  B  C  D  E  F  G  H   (x)
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_y, rank) in self.board.piece_array.iter().enumerate() {
            write!(f, "\n");
            for (_x, p) in rank.iter().enumerate() {
                if let Some(piece) = p {
                    write!(f, " {}", piece.char());
                } else {
                    write!(f, " *");
                }
            }
        }
        write!(f, "\n")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------
