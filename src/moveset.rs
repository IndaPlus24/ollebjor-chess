use std::{ops::Add, usize};

use crate::*;
use Move::*;

///Moveset
pub struct Moveset {
    pub moves: Vec<Move>,
    pub steps: usize,
    pub collide: bool
}

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
    fn get_position(&self, position: &BoardPosition) -> BoardPosition {
        let x = usize::from(position.x);
        let y = usize::from(position.y);
        let (max, min) = (7usize, 0usize);
        match self {
           Up => BoardPosition::from_num(x, (y+1).clamp(min, max)),
           Down => BoardPosition::from_num(x, (y-1).clamp(min, max)),
           Right => BoardPosition::from_num((x+1).clamp(min, max), y),
           Left => BoardPosition::from_num((x-1).clamp(min, max), y),
           UpRight => BoardPosition::from_num((x+1).clamp(min, max), (y+1).clamp(min, max)),
           UpLeft => BoardPosition::from_num((x-1).clamp(min, max), (y+1).clamp(min, max)),
           DownRight => BoardPosition::from_num((x+1).clamp(min, max), (y-1).clamp(min, max)),
           DownLeft => BoardPosition::from_num((x-1).clamp(min, max), (y-1).clamp(min, max)),
           Forward(color) => {
            match color {
                Color::Black => BoardPosition::from_num(x, (y-1).clamp(min, max)),
                Color::White => BoardPosition::from_num(x, (y+1).clamp(min, max))
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

    pub fn get_pseudo_legal(&self, position: &BoardPosition) -> Vec<BoardPosition>{
        let mut pseudo_legal = vec![];

        for move_action in self.moves.iter() {
            




            // let mut new_move = move_action.get_position(position);
            // for i in 0..=self.steps {
            //     pseudo_legal.push(new_move);
            //     new_move = move_action.get_position(&new_move);
            // }
        }

        pseudo_legal
    }

}

pub fn get_moveset(piece: Piece, color: Option<Color>) -> Moveset {
    match piece {
        other => Moveset::new(0, vec![], false)
    }
}
