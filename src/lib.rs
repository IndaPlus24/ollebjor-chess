use std::{fmt};
use Piece::*;
mod board_position;
use board_position::*;
mod moveset;
use moveset::*;
mod board;
use board::*;
mod tests;

const MAX_STEPS: u8 = 100;
const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}
#[derive(Debug, PartialEq, Copy, Clone)]
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
enum ChessError {
    IllegalMove,
    IllegalSpawn,
    NoPiece
}


///Game
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    turn: Color,
    board: Board,
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
        let mut position = BoardPosition::from_num(0, 7);
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
            //position.x += 1;
            position.x = Rank::from(usize::from(position.x) + 1);
        }
        // Flytta ned ett steg
        position = BoardPosition::from_num(0, 6);

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn, color, &position).expect("Could not spawn piece");
            //position.x += 1
            position.x = Rank::from(usize::from(position.x) + 1);
        }

        let color = Color::Black;
        position = BoardPosition::from_num(0,0);

        // Svarta coolingar
        for p in piece_array {
            self.board.spawn_piece(p, color, &position).expect("Could not spawn piece");
            //position.x += 1
            position.x = Rank::from(usize::from(position.x) + 1);
        }

        // Flytta upp ett steg
        position = BoardPosition::from_num(0, 1);

        // svarta bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn, color, &position).expect("Could not spawn piece");
            //position.x += 1
            position.x = Rank::from(usize::from(position.x) + 1);
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) -> Result<GameState, ChessError> {
        //Get the pseudolegal moves for the piece
        if let Some((piece, color)) =  self.board.get_piece(from) {
            let moveset = moveset::get_moveset(piece, Some(color));
            


        } else {
            return Err(ChessError::NoPiece)
        }
        //Use the pseudolegal moves for the piece to get the legal moves (dont forget to check check)
        //If to is a position to move to then move then remove any eventual piece that is there, track it, and move the piece there

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
    pub fn get_possible_moves(&self, postion: &BoardPosition) -> Option<Vec<BoardPosition>> {
        if let Some(piece_color_pair) =  self.board.get_piece(postion) {
            // get the moves for the piece
            let moveset = get_moveset(piece_color_pair.0, Some(piece_color_pair.1));
            
            let legal_moves: Vec<moveset::Move> = vec![];
            // get the legal moves for the piece based on the board
            for psuedo_legal_move in moveset.translations {
                
            }

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
        for col in self.board.position_array { 
            for piece in col {
                if piece.is_some() {
                    let c = piece.unwrap().0.to_char(&piece.unwrap().1);
                    write!(f, "{c}");
                } else {
                    write!(f, "*");
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
