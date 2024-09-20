use std::{fmt};
use Piece::*;

const MAX_STEPS: u8 = 100;

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
    Pawn(Color),
    Knight(Color),
    King(Color),
    Queen(Color),
    Bishop(Color),
    Rook(Color),
}

impl Piece {
    /// Returns a char based on the color and class of the piece
    pub fn to_char(&self) -> char {
        /// Makes the char uppercase if white, lowercase if black, if it errors. f will be returned
        fn color_case(char: char, color: &Color) -> char {
            match color {
                Color::Black => char.to_lowercase().next().unwrap_or('f'),
                Color::White => char.to_uppercase().next().unwrap_or('F'),
            }
        }

        // matches the colored piece to the right char 
        match self {
            Pawn(c) => color_case('p', c),
            Knight(c) => color_case('n', c),
            King(c) => color_case('k', c),
            Queen(c) => color_case('q', c),
            Bishop(c) => color_case('b', c),
            Rook(c) => color_case('r', c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y
        }
    }
}

impl From<usize> for Position {
    fn from(value: usize ) -> Self {
        Position {
            x: value,
            y: value
        }
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {

        let x = value.chars().next().unwrap_or('Ö');
        let y = value.chars().next().unwrap_or('Ö');

        fn char_to_num(c: char) -> usize {
            match c.to_uppercase().next().unwrap_or('Ö') {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                'E' => 5,
                'F' => 6,
                'G' => 7,
                'H' => 8,
                _ => 0
            }
        }

        Position::new(char_to_num(x), char_to_num(y))
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize))-> Self {
        Position::new(value.0, value.1)
    }
}

#[derive(Debug)]
enum ChessError {
    IllegalMove,
    IllegalSpawn,
    NoPiece
}

/// Board
#[derive(Debug)]
pub struct Board {
    position_array: [[Option<Piece>; 8]; 8],
}

impl Board {
    fn new() -> Board {
        Board {
            position_array: [[None; 8]; 8],
        }
    }

    /// returns a reference to the piece in the specified position
    fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.position_array[position.y-1][position.x-1]
    }

    ///Spawns the specified piece in the specified position
    fn spawn_piece(&mut self, piece: Piece, position: &Position) -> Result<(), ChessError> {
        if self.get_piece(position).is_some() {
            return Err(ChessError::IllegalSpawn);
        }

        self.position_array[position.y-1][position.x-1] = Some(piece);
        Ok(())
    }

    ///Removes the piece from the specified location
    fn despawn_piece(&mut self, position: Position) {
        self.position_array[position.y-1][position.x-1] = None;
    }

    fn clear(&mut self) {
        self.position_array = [[None; 8];8]
    }
}

///Moveset

pub struct Moveset {
    piece: Piece,
    max_steps: usize,
    translations: Vec<Piece>
}

impl Moveset {

}


///Game
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    turn: Color,
    board: Board,
    movesets: Vec<Moveset>
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
            movesets: vec![],
        }
    }

    //TODO: Do proper error handling
    pub fn init(&mut self) {
        // Vita pjäser
        //TODO: Handle results
        let color = Color::White;
        let mut position = Position::new(1, 8);
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
            self.board.spawn_piece(p, &position).expect("Could not spawn piece");
            position.x += 1;
        }
        // Flytta ned ett steg
        position = Position::new(1, 7);

        // Vita bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn(color), &position).expect("Could not spawn piece");
            position.x += 1
        }

        let color = Color::Black;
        position = Position::from(1);
        // Shadow array to change the color
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

        // Svarta coolingar
        for p in piece_array {
            self.board.spawn_piece(p, &position).expect("Could not spawn piece");
            position.x += 1
        }

        // Flytta upp ett steg
        position = Position::new(1, 2);

        // svarta bondlurkar
        for _ in 1..=8 {
            self.board.spawn_piece(Pawn(color), &position).expect("Could not spawn piece");
            position.x += 1
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: &Position, to: &Position) -> Result<GameState, ChessError> {

        
        if let Some(piece) =  self.board.get_piece(from) {
            // get the moves for the piece

            // Create a moveset struct that has a piece and an array of positions the piece can add to its current position.
            // Also could contain any special functions, for king and pawn
            // Also maybe a max_length. or max_steps

        }

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
    pub fn get_possible_moves(&self, postion: Position) -> Option<Vec<String>> {
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
                    let c = piece.unwrap().to_char();
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

#[cfg(test)]
mod tests {
    use crate::Color;
    use crate::Position;

    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    #[test]
    fn white_is_first() {
        let game = Game::new();

        assert_eq!(game.turn, Color::White);
    }

    #[test]
    fn turn_is_changed_after_move() {
        let mut game = Game::new();

        println!("{:?}", game);

        game.make_move(&Position::from("E7"), &Position::from("E6")).expect("Expected to move pawn from E7 to E6");

    }
}