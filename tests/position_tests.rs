use olle_chess::position::*;
//ChatGPT wrote these tests to test Position, BoardPosition, File and Rank functionality

#[test]
fn test_rank_conversion_usize() {
    assert_eq!(usize::from(Rank::One), 0);
    assert_eq!(usize::from(Rank::Two), 1);
    assert_eq!(usize::from(Rank::Eight), 7);
}

#[test]
fn test_rank_conversion_from_usize() {
    assert_eq!(Rank::try_from(0).unwrap(), Rank::One);
    assert_eq!(Rank::try_from(7).unwrap(), Rank::Eight);
}

#[test]
#[should_panic]
fn test_rank_conversion_invalid_usize() {
    let _ = Rank::try_from(8).unwrap();  // This should panic
}

#[test]
fn test_file_conversion_usize() {
    assert_eq!(usize::from(File::A), 0);
    assert_eq!(usize::from(File::H), 7);
}

#[test]
fn test_file_conversion_from_usize() {
    assert_eq!(File::try_from(0).unwrap(), File::A);
    assert_eq!(File::try_from(7).unwrap(), File::H);
}

#[test]
#[should_panic]
fn test_file_conversion_invalid_usize() {
    let _ = File::try_from(8).unwrap();  // This should panic
}

#[test]
fn test_board_position_new() {
    let position = BoardPosition::new(File::A, Rank::One);
    assert_eq!(position.file, File::A);
    assert_eq!(position.rank, Rank::One);
}

#[test]
fn test_position_new_valid() {
    let position = Position::new(3, 5);
    assert_eq!(position.x, 3);
    assert_eq!(position.y, 5);
}


#[test]
fn test_position_to_board_position() {
    let pos = Position::new(3, 5);
    let board_pos: BoardPosition = pos.try_into().unwrap();
    assert_eq!(board_pos.file, File::D);
    assert_eq!(board_pos.rank, Rank::Six);
}

#[test]
fn test_board_position_to_position() {
    let board_pos = BoardPosition::new(File::E, Rank::Seven);
    let pos: Position = board_pos.into();
    assert_eq!(pos.y, 6);
    assert_eq!(pos.x, 4);
}

#[test]
fn test_board_position_from_str() {
    let board_pos: BoardPosition = BoardPosition::try_from("E2").unwrap();
    assert_eq!(board_pos.file, File::E);
    assert_eq!(board_pos.rank, Rank::Two);
}

#[test]
fn test_position_multiplication() {
    let pos = Position::new(2, 3);
    let result = pos * 2;
    assert_eq!(result.x, 4);
    assert_eq!(result.y, 6);
}
