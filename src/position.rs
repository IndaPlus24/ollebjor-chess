use core::fmt;
use std::{ops::Mul, usize};

use File::*;
use Rank::*;

use crate::ChessError;

///Rank
/// Rank is the enum representing the y-value of the board position. 
/// It consists of Let. (One, Two, ..., Seven, Eight)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

impl From<Rank> for usize {
    fn from(value: Rank) -> Self {
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

impl From<usize> for Rank {
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
            _ => panic!()
        }
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        match value {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8'
        }
    }
}

impl From<char> for Rank {
    fn from(value: char) -> Self {
        match value {
            '1' => One, 
            '2' => Two,
            '3' => Three,
            '4' => Four, 
            '5' => Five, 
            '6' => Six, 
            '7' => Seven,
            '8' => Eight,
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl From<File> for usize {
    fn from(value: File) -> Self {
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

impl From<usize> for File {
    fn from(value: usize) -> File{
        match value {
            0 => A, 
            1 => B,
            2 => C,
            3 => D, 
            4 => E, 
            5 => F, 
            6 => G,
            7 => H,
            _ => panic!()
        }
    }
}

impl From<File> for char {
    fn from(value: File) -> Self {
        match value {
            A => 'A', 
            B => 'B',
            C => 'C',
            D => 'D', 
            E => 'E', 
            F => 'F', 
            G => 'G',
            H => 'H',
        }
    }
}

impl From<char> for File {
    fn from(value: char) -> Self {
        match value {
            'A' => A, 
            'B' => B,
            'C' => C,
            'D' => D, 
            'E' => E, 
            'F' => F, 
            'G' => G,
            'H' => H,
            _ => panic!()
        }
    }
}
/// BoardPosition is the representation of a position on a position board
/// *Ranks* => *rows*
/// *File* => *columns*
/// 
#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct BoardPosition {
    pub file: File,
    pub rank: Rank
}

impl BoardPosition {
    pub fn new(file: File, rank: Rank) -> Self {
        BoardPosition {
            file,
            rank,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Result<Self, ChessError> {
        if x > 7 || y > 7 {
            return Err(ChessError::OutOfBounds);
        }

        Ok(Position {
            x,
            y,
        })
    }
}

impl From<BoardPosition> for Position {
    fn from(value: BoardPosition) -> Self {
        Position { 
            x: value.rank.into(), 
            y: value.file.into()
        }
    }
}

impl From<Position> for BoardPosition {
    fn from(value: Position) -> Self {
        BoardPosition {
            file: File::from(value.x),
            rank: Rank::from(value.y) 
        }
    }
}

impl From<&str> for BoardPosition {
    fn from(value: &str) -> Self {
        let value = value.trim().to_uppercase();

        if value.len() != 2 {
            panic!("POSITION STRING IS NOT 2 LONG")
        }
        println!("str is: {value}");
        let mut chars = value.chars();
        let file = chars.next().unwrap_or('Ö');
        let rank = chars.next().unwrap_or('Ö');
        
        println!("{:?} {:?}", file, rank);
        println!("{:?}", BoardPosition::new(file.into(), rank.into()));
        BoardPosition::new(file.into(), rank.into())
    }
}


impl Mul<usize> for Position {
    type Output = Position;
    fn mul(self, rhs: usize) -> Self::Output {
        Position::new(self.x * rhs, self.y * rhs).unwrap()
    }
}

impl fmt::Debug for BoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file: char = self.file.into();
        let rank: char = self.rank.into();
        write!(f, "{}{}", file, rank)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}