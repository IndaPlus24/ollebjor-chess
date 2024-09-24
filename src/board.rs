use crate::*;
/// Board
/// Board is the struct for raw board util behaviour such as placing pieces, removing pieces, getting pieces.
#[derive(Debug)]
pub struct Board {
    pub position_array: [[Option<(Piece, Color)>; BOARD_SIZE]; BOARD_SIZE],
}
// Board has a 2D array that is first indexed by the rank then the file.
// This is so that I can loop through a whole row, instead of columns.
impl Board {
    pub fn new() -> Board {
        Board {
            position_array: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// returns a reference to the piece in the specified position
    pub fn get_piece_ref(&self, position: &BoardPosition) -> &Option<(Piece, Color)> {
        &self.position_array[usize::from(position.y)][usize::from(position.x)]
    }

    pub fn get_piece(&self, position: &BoardPosition) -> Option<(Piece, Color)> {
        self.position_array[usize::from(position.y)][usize::from(position.x)]
    }

    ///Spawns the specified piece in the specified position, 
    /// gives a result that holds a ChessError if there is already a piece in that posistion.
    /// Use set_piece if you dont want this chesserror behaviour
    pub fn spawn_piece(&mut self, piece: Piece, color: Color,  position: &BoardPosition) -> Result<(), ChessError> {
        if self.get_piece(position).is_some() {
            return Err(ChessError::IllegalSpawn);
        }

        self.set_piece(piece, color, position);
        Ok(())
    }

    pub fn set_piece(&mut self, piece: Piece, color: Color, position: &BoardPosition) -> &Option<(Piece, Color)> {
        self.position_array[usize::from(position.y)][usize::from(position.x)] = Some((piece, color));
        self.get_piece_ref(position)
    }

    ///Removes the piece from the specified location
    pub fn despawn_piece(&mut self, position: BoardPosition) {
        self.position_array[usize::from(position.y)][usize::from(position.x)] = None;
    }

    pub fn clear(&mut self) {
        self.position_array = [[None; 8];8]
    }
}
