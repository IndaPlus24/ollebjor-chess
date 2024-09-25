use std::collections::HashMap;

use crate::*;
use Move::*;

///Moveset
pub struct Moveset {
    pub moves: Vec<Move>,
    pub steps: usize,
    pub collide: bool
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
    pub fn get_position(&self, position: &BoardPosition, steps: usize) -> BoardPosition {
        let x = usize::from(position.x);
        let y = usize::from(position.y);
        let (max, min) = (7usize, 0usize);
        match self {
           Up => BoardPosition::from_usize(x, (y+steps).clamp(min, max)),
           Down => BoardPosition::from_usize(x, (y-steps).clamp(min, max)),
           Right => BoardPosition::from_usize((x+steps).clamp(min, max), y),
           Left => BoardPosition::from_usize((x-steps).clamp(min, max), y),
           UpRight => BoardPosition::from_usize((x+steps).clamp(min, max), (y+steps).clamp(min, max)),
           UpLeft => BoardPosition::from_usize((x-steps).clamp(min, max), (y+steps).clamp(min, max)),
           DownRight => BoardPosition::from_usize((x+steps).clamp(min, max), (y-steps).clamp(min, max)),
           DownLeft => BoardPosition::from_usize((x-steps).clamp(min, max), (y-steps).clamp(min, max)),
           Forward(color) => {
            match color {
                Color::Black => BoardPosition::from_usize(x, (y-steps).clamp(min, max)),
                Color::White => BoardPosition::from_usize(x, (y+steps).clamp(min, max))
            }
           }
       }
    }
}

impl Moveset {
    pub fn new(steps: usize, moves: Vec<Move>, collide: bool) -> Self {
        Moveset {
            steps,
            moves,
            collide,
        }
    }
}

pub fn get_moveset(piece: Piece, color: Option<Color>) -> Moveset {
    match piece {
        other => Moveset::new(0, vec![], false)
    }
}
