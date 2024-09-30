use core::fmt;
use std::{ops::Mul, usize};

use File::*;
use Rank::*;

use crate::ChessError;

///Rank
/// Rank is the enum representing the y-value of the board position. 
/// It consists of Let. (One, Two, ..., Seven, Eight)
/// Internally these are switched, (One=>7, Eight =>0)
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

impl TryFrom<usize> for Rank {
    type Error = ChessError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(One), 
            1 => Ok(Two),
            2 => Ok(Three), 
            3 => Ok(Four), 
            4 => Ok(Five), 
            5 => Ok(Six), 
            6 => Ok(Seven),
            7 => Ok(Eight),
            _ => Err(ChessError::InvalidRank),
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

impl TryFrom<char> for Rank {
    type Error = ChessError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            _ => Err(ChessError::InvalidRank),
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

impl TryFrom<usize> for File {
    type Error = ChessError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(A), 
            1 => Ok(B),
            2 => Ok(C),
            3 => Ok(D), 
            4 => Ok(E), 
            5 => Ok(F), 
            6 => Ok(G),
            7 => Ok(H),
            _ => Err(ChessError::InvalidFile)
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

impl TryFrom<char> for File {
    type Error = ChessError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(A), 
            'B' => Ok(B),
            'C' => Ok(C),
            'D' => Ok(D), 
            'E' => Ok(E), 
            'F' => Ok(F), 
            'G' => Ok(G),
            'H' => Ok(H),
            _ => Err(ChessError::InvalidFile)
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
    pub fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y,
        }
    }
}


impl From<BoardPosition> for Position {
    fn from(value: BoardPosition) -> Position {
        Position { 
            x: usize::from(value.file), 
            y: usize::from(value.rank)
        }
    }
}

impl From<&BoardPosition> for Position {
    fn from(value: &BoardPosition) -> Position {
        Position { 
            x: usize::from(value.file), 
            y: usize::from(value.rank)
        }
    }
}

impl TryFrom<Position> for BoardPosition {
    type Error = ChessError;
    fn try_from(value: Position) -> Result<Self, Self::Error> {
        Ok(BoardPosition {
            file: File::try_from(value.x)?,
            rank: Rank::try_from(value.y)?
        })
    }
}

impl TryFrom<&Position> for BoardPosition {
    type Error = ChessError;
    fn try_from(value: &Position) -> Result<Self, Self::Error> {
        Ok(BoardPosition {
            file: File::try_from(value.x)?,
            rank: Rank::try_from(value.y)?
        })
    }
}

impl TryFrom<(usize, usize)> for BoardPosition {
    type Error = ChessError;
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        Ok(BoardPosition {
            file: File::try_from(value.0)?,
            rank: Rank::try_from(value.1)?
        })
    }
}

impl TryFrom<&str> for BoardPosition {
    type Error = ChessError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().to_uppercase();

        if value.len() != 2 {
            return Err(ChessError::InvalidPositionString);
        }

        let mut chars = value.chars();
        let file = chars.next().ok_or(ChessError::InvalidPositionString)?;
        let rank = chars.next().ok_or(ChessError::InvalidPositionString)?;

        Ok(BoardPosition::new(File::try_from(file)?, Rank::try_from(rank)?))
    }
}


impl Mul<usize> for Position {
    type Output = Position;
    fn mul(self, rhs: usize) -> Self::Output {
        Position::new(self.x * rhs, self.y * rhs)
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