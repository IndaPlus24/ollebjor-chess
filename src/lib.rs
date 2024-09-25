use std::{fmt};
use Piece::*;
pub mod position;
use position::*;
pub mod moveset;
use moveset::*;
pub mod board;
use board::*;

const MAX_STEPS: u8 = 100;
const BOARD_SIZE: usize = 8;

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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    King,
    Queen,
    Bishop,
    Rook,
}

impl Piece {
    /// Returns a char based on the color and class of the piece
    pub fn to_char(&self, color: &Color) -> char {
        /// Makes the char uppercase if white, lowercase if black, if it errors. f will be returned
        fn color_case(char: char, color: &Color) -> char {
            match color {
                Color::Black => char.to_lowercase().next().unwrap_or('f'),
                Color::White => char.to_uppercase().next().unwrap_or('F'),
            }
        }

        // matches the colored piece to the right char 
        match self {
            Pawn => color_case('p', color),
            Knight => color_case('n', color),
            King => color_case('k', color),
            Queen => color_case('q', color),
            Bishop => color_case('b', color),
            Rook => color_case('r', color),
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
        let color = Color::White;
        let mut position = BoardPosition::from_usize(0, 7);
        let piece_array = [
            Rook,
            Knight,
            Bishop,
            King,
            Queen,
            Bishop,
            Knight,
            Rook,
        ];

        // Vita coola grabbar
        for p in piece_array {
            self.board.spawn_piece(p, color,&position).expect("Could not spawn piece");
            position.x += 1;
        }
        // Flytta ned ett steg
        position = BoardPosition::from_usize(0, 6);

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn, color, &position).expect("Could not spawn piece");
            position.x += 1
        }

        let color = Color::Black;
        position = BoardPosition::from_usize(0,0);

        // Svarta coolingar
        for p in piece_array {
            self.board.spawn_piece(p, color, &position).expect("Could not spawn piece");
            position.x += 1
        }

        // Flytta upp ett steg
        position = BoardPosition::from_usize(0, 1);

        // svarta bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn, color, &position).expect("Could not spawn piece");
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
        //Get the piece
        if let Some((piece, color)) =  self.board.get_piece(position) {
           let moveset = moveset::get_moveset(piece, Some(color));

           let mut legal_moves: Vec<BoardPosition> = vec![];
           for move_action in moveset.moves.into_iter() {

            'step: for step in 1..=moveset.steps {
                println!("Step: {step}");
                //Check to see if there is a piece on this place
                let next_step = move_action.get_position(position, step);
                println!("Position: {:?}", next_step);
                if let Some((_p, c)) = self.board.get_piece(&next_step) {
                    println!("{c:?}");
                    println!("{_p:?}");
                    if c == color {
                        break 'step;
                    } else {
                        //push then break
                        legal_moves.push(next_step);
                        break 'step;
                    }
                } else {
                    //Push this step
                    legal_moves.push(next_step);
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
        /* build board representation string */

        write!(f, "\n");
        for row in self.board.position_array { 
            for piece in row {
                if piece.is_some() {
                    let c = piece.unwrap().0.to_char(&piece.unwrap().1);
                    write!(f, "{c} ");
                } else {
                    write!(f, "* ");
                }
            }
            write!(f, "\n");
        }

        write!(f, "\n")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------
