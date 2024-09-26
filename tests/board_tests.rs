use olle_chess::*;
use olle_chess::board::Board;
use olle_chess::position::*;

//ChatGPT skrev dessa tester också lol

fn sample_piece() -> Piece {
    // Assuming you have some Piece struct with an enum or other definition.
    // Replace this with actual piece creation logic.
    Piece::Pawn(Color::White)
}

#[test]
fn test_board_initialization() {
    let board = Board::new();
    for row in &board.piece_array {
        for piece in row {
            assert!(piece.is_none());
        }
    }
}

#[test]
fn test_spawn_piece_on_empty_square() {
    let mut board = Board::new();
    let position = Position::new(3, 4).unwrap();
    let piece = sample_piece();

    assert!(board.spawn_piece(piece, &position).is_ok());
    assert_eq!(board.get_piece(&position), Some(piece));
}

#[test]
fn test_spawn_piece_on_occupied_square() {
    let mut board = Board::new();
    let position = Position::new(3, 4).unwrap();
    let piece1 = sample_piece();
    let piece2 = sample_piece();

    board.spawn_piece(piece1, &position).unwrap();
    let result = board.spawn_piece(piece2, &position);

    assert!(result.is_err());  // Should return ChessError::IllegalSpawn
    assert_eq!(board.get_piece(&position), Some(piece1)); // Original piece remains
}

#[test]
fn test_set_piece() {
    let mut board = Board::new();
    let position = Position::new(3, 4).unwrap();
    let piece = sample_piece();

    board.set_piece(piece, &position);
    assert_eq!(board.get_piece(&position), Some(piece));
}

#[test]
fn test_despawn_piece() {
    let mut board = Board::new();
    let position = Position::new(3, 4).unwrap();
    let piece = sample_piece();

    board.spawn_piece(piece, &position).unwrap();
    board.despawn_piece(&position);

    assert_eq!(board.get_piece(&position), None);
}

#[test]
fn test_clear_board() {
    let mut board = Board::new();
    let piece = sample_piece();

    for x in 0..board::BOARD_SIZE {
        for y in 0..board::BOARD_SIZE {
            let position = Position::new(x, y).unwrap();
            board.set_piece(piece, &position);
        }
    }

    board.clear();

    for x in 0..board::BOARD_SIZE {
        for y in 0..board::BOARD_SIZE {
            let position = Position::new(x, y).unwrap();
            assert!(board.get_piece(&position).is_none());
        }
    }
}

#[test]
fn test_get_piece_ref() {
    let mut board = Board::new();
    let position = Position::new(3, 4).unwrap();
    let piece = sample_piece();

    board.spawn_piece(piece, &position).unwrap();
    let piece_ref = board.get_piece_ref(&position);

    assert_eq!(piece_ref.as_ref(), Some(&piece));
}

#[test]
fn test_board_display() {
    let board = Board::new();
    let board_display = format!("{}", board);
    // This assumes the board display will be a grid of positions.
    // You can adjust this to match the exact output format of the board.
    assert!(board_display.contains("A1"));
    assert!(board_display.contains("H8"));
}

