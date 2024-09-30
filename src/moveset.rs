use crate::*;
use Move::*;

///Moveset
pub struct Moveset {
    pub moves: Vec<Move>,
    pub steps: usize,
    pub jumps: bool
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Move {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    KnightUpRight,
    KnightRightUp,
    KnightUpLeft,
    KnightLeftUp,
    KnightDownRight,
    KnightRightDown,
    KnightDownLeft,
    KnightLeftDown,
    Forward(Color),
}

impl Move {
    pub fn get_position(&self, position: &Position, steps: usize) -> Result<Position, ChessError> {
        let (max, min) = (7, 0);
        let x = position.x as i8;
        let y = position.y as i8;

        let (x, y):(i8, i8) = match self {
           Up => (x, y + steps as i8),
           Down => (x, y - steps as i8),
           Right => (x + steps as i8, y),
           Left => (x - steps as i8, y),
           UpRight => (x + steps as i8, y + steps as i8),
           UpLeft => (x - steps as i8, y + steps as i8),
           DownRight => (x + steps as i8, y - steps as i8),
           DownLeft => (x - steps as i8, y - steps as i8),
           KnightUpRight => (x + steps as i8, y + (steps * 2) as i8),
           KnightUpLeft => (x - steps as i8, y + (steps * 2) as i8),
           KnightDownRight => (x + steps as i8, y - (steps * 2) as i8),
           KnightDownLeft => (x - steps as i8, y - (steps * 2) as i8),
           KnightRightUp => (x + (steps * 2) as i8, y + steps as i8),
           KnightLeftUp => (x - (steps * 2) as i8, y + steps as i8),
           KnightRightDown => (x + (steps * 2) as i8, y - steps as i8),
           KnightLeftDown => (x - (steps * 2) as i8, y - steps as i8),
           Forward(color) => 
            match color {
                Color::Black => (x, y-steps as i8),
                Color::White => (x, y+steps as i8),
            }
           };
           if x < min || x > max || y < min || y > max {
               Err(ChessError::OutOfBounds)
           } else {
           Ok(Position::new(x as usize, y as usize))
           }
        }
    }

impl Moveset {
    pub fn new(steps: usize, moves: Vec<Move>, jumps: bool) -> Self {
        Moveset {
            steps,
            moves,
            jumps,
        }
    }
}

pub fn get_moveset(piece: Piece) -> Moveset {
    match piece {
        Pawn(c) => Moveset::new(1, vec![Forward(c)], false),
        Rook(_) => Moveset::new(7, vec![Up, Down, Left, Right], false),
        Knight(_) => Moveset::new(1, vec![
            KnightUpRight,
            KnightRightUp,
            KnightUpLeft,
            KnightLeftUp,
            KnightDownRight,
            KnightRightDown,
            KnightDownLeft,
            KnightLeftDown
        ], true),
        Bishop(_) => Moveset::new(7, vec![UpRight, UpLeft, DownRight, DownLeft], false),
        Queen(_) => Moveset::new(7, vec![Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft], false),
        King(_) => Moveset::new(1, vec![Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft], false),
    }
}
