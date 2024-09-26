use crate::*;
pub const BOARD_SIZE: usize = 8;
/// Board
/// Board is the struct for raw board util behaviour such as placing pieces, removing pieces, getting pieces.
pub struct Board {
    pub piece_array: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}
// Board has a 2D array that is first indexed by the rank then the file.
// This is so that I can loop through a whole row, instead of columns.
impl Board {
    pub fn new() -> Board {
        Board {
            piece_array: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// returns a reference to the piece in the specified position
    pub fn get_piece_ref(&self, position: &Position) -> &Option<Piece> {
        &self.piece_array[(7usize).abs_diff(position.y)][position.x]
    }

    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.piece_array[(7usize).abs_diff(position.y)][position.x]
    }

    /// Spawns the specified piece in the specified position, 
    /// gives a result that holds a ChessError if there is already a piece in that posistion.
    /// Use set_piece if you dont want this chesserror behaviour
    pub fn spawn_piece(&mut self, piece: Piece, position: &Position) -> Result<(), ChessError> {
        if self.get_piece(position).is_some() {
            return Err(ChessError::IllegalSpawn);
        }

        self.set_piece(piece, position);
        Ok(())
    }

    /// Sets the piece in the specified position without checking if there is a piece in that position
    pub fn set_piece(&mut self, piece: Piece, position: &Position) -> &Option<Piece> {
        self.piece_array[(7usize).abs_diff(position.y)][position.x] = Some(piece);
        self.get_piece_ref(position)
    }

    ///Removes the piece from the specified location
    pub fn despawn_piece(&mut self, position: &Position) {
        self.piece_array[(7usize).abs_diff(position.y)][position.x] = None;
    }

    pub fn clear(&mut self) {
        self.piece_array = [[None; BOARD_SIZE]; BOARD_SIZE]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, rank) in self.piece_array.iter().enumerate() {
            write!(f, "\n")?;
            for (x, _file) in rank.iter().enumerate() {
                let pos: BoardPosition = Position::new(x, (7usize).abs_diff(y)).unwrap().into();
                write!(f, " {pos:?}")?;
            }
        }
        write!(f, "\n")
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, rank) in self.piece_array.iter().enumerate() {
            write!(f, "\n")?;
            for (x, _file) in rank.iter().enumerate() {
                write!(f, "[{}][{}] ", x, y)?;
            }
        }
        write!(f, "\n")
    }
}