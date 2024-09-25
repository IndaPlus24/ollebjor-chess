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
    assert_eq!(Rank::from(0), Rank::One);
    assert_eq!(Rank::from(7), Rank::Eight);
}

#[test]
#[should_panic]
fn test_rank_conversion_invalid_usize() {
    let _ = Rank::from(8);  // This should panic
}

#[test]
fn test_file_conversion_usize() {
    assert_eq!(usize::from(File::A), 0);
    assert_eq!(usize::from(File::H), 7);
}

#[test]
fn test_file_conversion_from_usize() {
    assert_eq!(File::from(0), File::A);
    assert_eq!(File::from(7), File::H);
}

#[test]
#[should_panic]
fn test_file_conversion_invalid_usize() {
    let _ = File::from(8);  // This should panic
}

#[test]
fn test_board_position_new() {
    let position = BoardPosition::new(File::A, Rank::One);
    assert_eq!(position.file, File::A);
    assert_eq!(position.rank, Rank::One);
}

#[test]
fn test_position_new_valid() {
    let position = Position::new(3, 5).unwrap();
    assert_eq!(position.x, 3);
    assert_eq!(position.y, 5);
}

#[test]
fn test_position_new_out_of_bounds() {
    assert!(Position::new(8, 5).is_err());
    assert!(Position::new(3, 8).is_err());
}

#[test]
fn test_position_to_board_position() {
    let pos = Position::new(3, 5).unwrap();
    let board_pos: BoardPosition = pos.into();
    assert_eq!(board_pos.file, File::D);
    assert_eq!(board_pos.rank, Rank::Six);
}

#[test]
fn test_board_position_to_position() {
    let board_pos = BoardPosition::new(File::E, Rank::Seven);
    let pos: Position = board_pos.into();
    assert_eq!(pos.x, 6);
    assert_eq!(pos.y, 4);
}

#[test]
fn test_board_position_from_str() {
    let board_pos: BoardPosition = "E2".into();
    assert_eq!(board_pos.file, File::E);
    assert_eq!(board_pos.rank, Rank::Two);
}

#[test]
fn test_position_multiplication() {
    let pos = Position::new(2, 3).unwrap();
    let result = pos * 2;
    assert_eq!(result.x, 4);
    assert_eq!(result.y, 6);
}
