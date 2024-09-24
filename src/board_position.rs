use std::{i8, ops::Mul};

use File::*;
use Rank::*;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum File {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

impl From<File> for usize {
    fn from(value: File) -> Self {
        match value {
            One => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7
        }
    }
}
impl From<File> for i8 {
    fn from(value: File) -> Self {
        match value {
            One => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7
        }
    }
}
impl From<usize> for File {
    fn from(value: usize) -> Self {
        match value {
            0 => One, 
            1 => Two,
            2 => Three,
            3 => Four, 
            4 => Five, 
            5 => Six, 
            6 => Seven,
            7 => Eight,
            big => File::from(big % 8)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl From<Rank> for usize {
    fn from(value: Rank) -> Self {
        match value {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            H => 7
        }
    }
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        match value {
            0 => A, 
            1 => B,
            2 => C,
            3 => D, 
            4 => E, 
            5 => F, 
            6 => G,
            7 => H,
            big => Rank::from(big % 8)
        }
    }
}

impl From<Rank> for i8 {
    fn from(value: Rank) -> Self {
        match value {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            H => 7
        }
    }
}

impl From<i8> for usize {
    fn from(value: i8) -> Self {
        value.clamp(0, i8::MAX) as usize
    }
}

/// BoardPosition is the representation of a position on a position board
/// *Ranks* => *rows*
/// *File* => *columns*
/// 
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    pub x: i8,
    pub y: i8
}

impl BoardPosition {
    pub fn new(x: Rank, y: File) -> Self {
        BoardPosition {
            x: i8::from(x),
            y: i8::from(y)
        }
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        BoardPosition::new(x.into(), y.into())
    }

    pub fn from_i8(x: i8, y: i8) -> Self {
        BoardPosition{
            x,
            y
        }
    }

}

impl From<&str> for BoardPosition {
    fn from(value: &str) -> Self {
        let value = value.trim();

        if value.len() != 2 {
            panic!("POSITION STRING IS NOT 2 LONG")
        }

        let x = value.chars().next().unwrap_or('Ö');
        let y = value.chars().next().unwrap_or('Ö');

        fn char_to_num(c: char) -> usize {
            match c.to_uppercase().next().unwrap_or('Ö') {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                _ => 0
            }
        }

        BoardPosition::from_usize(char_to_num(x), char_to_num(y))
    }
}

impl Mul<i8> for BoardPosition {
    type Output = BoardPosition;
    fn mul(self, rhs: i8) -> Self::Output {
        BoardPosition::from_i8(self.x * rhs, self.y * rhs)
    }
}