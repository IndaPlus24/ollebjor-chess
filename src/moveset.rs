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
    Forward(Color),
}

impl Move {
    pub fn get_position(&self, position: &Position, steps: usize) -> Position {
        let x = position.x;
        let y = position.y;
        let (max, min) = (7usize, 0usize);
        let pos = match self {
           Up => Position::new(x, (y+steps).clamp(min, max)),
           Down => Position::new(x, (y-steps).clamp(min, max)),
           Right => Position::new((x+steps).clamp(min, max), y),
           Left => Position::new((x-steps).clamp(min, max), y),
           UpRight => Position::new((x+steps).clamp(min, max), (y+steps).clamp(min, max)),
           UpLeft => Position::new((x-steps).clamp(min, max), (y+steps).clamp(min, max)),
           DownRight => Position::new((x+steps).clamp(min, max), (y-steps).clamp(min, max)),
           DownLeft => Position::new((x-steps).clamp(min, max), (y-steps).clamp(min, max)),
           Forward(color) => {
            match color {
                Color::Black => Position::new(x, (y-steps).clamp(min, max)),
                Color::White => Position::new(x, (y+steps).clamp(min, max))
            }
           }
        };
        pos.unwrap()
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
        Pawn(c) => Moveset::new(1, vec![Forward(c)], true),
        _ => Moveset::new(0, vec![], false),
    }
}
