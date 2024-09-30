use crate::*;
pub const BOARD_SIZE: usize = 8;
/// Board
/// Board is the struct for raw board util behaviour such as placing pieces, removing pieces, getting pieces.
pub struct Board {
    pub piece_array: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub white_king_position: Option<Position>,
    pub black_king_position: Option<Position>,
}
// Board has a 2D array that is first indexed by the rank then the file.
// This is so that I can loop through a whole row, instead of columns.
impl Board {
    pub fn new() -> Board {
        Board {
            piece_array: [[None; BOARD_SIZE]; BOARD_SIZE],
            white_king_position: None,
            black_king_position: None,
        }
    }

    pub fn get_rank(&self, rank: Rank) -> [Option<Piece>; BOARD_SIZE] {
        let y = usize::from(rank);
        self.piece_array[(7usize).abs_diff(y)]
    }

    pub fn get_file(&self, file: File) -> [Option<Piece>; BOARD_SIZE] {
        let x = usize::from(file);
        let mut file_array = [None; BOARD_SIZE];
        for (y, rank) in self.piece_array.iter().enumerate() {
            file_array[(7usize).abs_diff(y)] = rank[x];
        }
        file_array
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
        if let King(color) = piece {
            match color {
                Color::White => self.white_king_position = Some(*position),
                Color::Black => self.black_king_position = Some(*position),
            }
        }
        self.piece_array[(7usize).abs_diff(position.y)][position.x] = Some(piece);
        self.get_piece_ref(position)
    }

    ///Removes the piece from the specified location
    /// If it is the king, the king position is also removed
    pub fn despawn_piece(&mut self, position: &Position) {
        //check to see if the piece is a king and remove the king position
        if let Some(piece) = self.get_piece(position) {
            if let King(color) = piece {
                match color {
                    Color::White => self.white_king_position = None,
                    Color::Black => self.black_king_position = None,
                }
            }
        }
        self.piece_array[(7usize).abs_diff(position.y)][position.x] = None;
    }

    ///Moves the piece from the start position to the end position
    /// If there is a piece in the end position, it is removed
    pub fn move_piece(&mut self, from: &Position, to: &Position) {
        if let Some(piece) = self.get_piece(from) {
            self.despawn_piece(to);
            self.set_piece(piece, to);
            self.despawn_piece(from);
        }
    }

    ///Clears the board of all pieces
    pub fn clear(&mut self) {
        self.piece_array = [[None; BOARD_SIZE]; BOARD_SIZE]
    }

    ///Returns the position of the king of the specified color
    pub fn get_king_position(&self, color: Color) -> Option<Position> {
        match color {
            Color::White => self.white_king_position,
            Color::Black => self.black_king_position,
        }
    }

    ///Returns all pieces on the board
    pub fn get_all_pieces(&self) -> Vec<&Piece> {
        let mut pieces = Vec::new();
        for rank in self.piece_array.iter() {
            for piece in rank.iter() {
                if let Some(p) = piece {
                    pieces.push(p);
                }
            }
        }
        pieces
    }

    ///Returns all pieces on the board of the specified color
    pub fn get_all_pieces_of_color(&self, color: Color) -> Vec<&Piece> {
        let mut pieces = self.get_all_pieces();
        pieces.retain(|p| p.get_color() == color);
        pieces
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, rank) in self.piece_array.iter().enumerate() {
            write!(f, "\n")?;
            for (x, _file) in rank.iter().enumerate() {
                let pos: BoardPosition = Position::new(x, (7usize).abs_diff(y)).try_into().unwrap();
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