use olle_chess::*;
use olle_chess::position::*;
use olle_chess::moveset::*;

fn get_initial_position() -> Position {
   Position::new(3, 3).unwrap()  // Sample position at D4
}

#[test]
fn test_pawn_moves() {
   let white_pawn = Piece::Pawn(Color::White);
   let black_pawn = Piece::Pawn(Color::Black);
   let position = get_initial_position();

   let white_moveset = get_moveset(white_pawn);
   let black_moveset = get_moveset(black_pawn);

   // Test white pawn moves forward
   let forward_white = white_moveset.moves[0].get_position(&position, 1);
   assert_eq!(forward_white, Position::new(3, 4).unwrap());

   // Test black pawn moves forward (which is actually backward in terms of board y-axis)
   let forward_black = black_moveset.moves[0].get_position(&position, 1);
   assert_eq!(forward_black, Position::new(3, 2).unwrap());
}

#[test]
fn test_rook_moves() {
   let rook = Piece::Rook(Color::White);
   let position = get_initial_position();

   let moveset = get_moveset(rook);

   for move_type in moveset.moves.iter() {
       // Test all 7 steps for rook's movement
       for step in 1..=7 {
           let new_position = move_type.get_position(&position, step);
           match move_type {
               Move::Up => assert_eq!(new_position, Position::new(3, 3 + step).unwrap()),
               Move::Down => assert_eq!(new_position, Position::new(3, 3 - step).unwrap()),
               Move::Right => assert_eq!(new_position, Position::new(3 + step, 3).unwrap()),
               Move::Left => assert_eq!(new_position, Position::new(3 - step, 3).unwrap()),
               _ => panic!("Invalid move for rook"),
           }
       }
   }
}

#[test]
fn test_knight_moves() {
   let knight = Piece::Knight(Color::White);
   let position = get_initial_position();

   let moveset = get_moveset(knight);

   let expected_positions = vec![
       Position::new(4, 5).unwrap(),  // KnightUpRight
       Position::new(2, 5).unwrap(),  // KnightUpLeft
       Position::new(4, 1).unwrap(),  // KnightDownRight
       Position::new(2, 1).unwrap(),  // KnightDownLeft
       Position::new(5, 4).unwrap(),  // KnightRightUp
       Position::new(1, 4).unwrap(),  // KnightLeftUp
       Position::new(5, 2).unwrap(),  // KnightRightDown
       Position::new(1, 2).unwrap(),  // KnightLeftDown
   ];

   for (i, move_type) in moveset.moves.iter().enumerate() {
       let new_position = move_type.get_position(&position, 1);
       assert_eq!(new_position, expected_positions[i]);
   }
}

#[test]
fn test_bishop_moves() {
   let bishop = Piece::Bishop(Color::White);
   let position = get_initial_position();

   let moveset = get_moveset(bishop);

   for move_type in moveset.moves.iter() {
       for step in 1..=7 {
           let new_position = move_type.get_position(&position, step);
           match move_type {
               Move::UpRight => assert_eq!(new_position, Position::new(3 + step, 3 + step).unwrap()),
               Move::UpLeft => assert_eq!(new_position, Position::new(3 - step, 3 + step).unwrap()),
               Move::DownRight => assert_eq!(new_position, Position::new(3 + step, 3 - step).unwrap()),
               Move::DownLeft => assert_eq!(new_position, Position::new(3 - step, 3 - step).unwrap()),
               _ => panic!("Invalid move for bishop"),
           }
       }
   }
}

#[test]
fn test_queen_moves() {
   let queen = Piece::Queen(Color::White);
   let position = get_initial_position();

   let moveset = get_moveset(queen);

   for move_type in moveset.moves.iter() {
       for step in 1..=7 {
           let new_position = move_type.get_position(&position, step);
           match move_type {
               Move::Up => assert_eq!(new_position, Position::new(3, 3 + step).unwrap()),
               Move::Down => assert_eq!(new_position, Position::new(3, 3 - step).unwrap()),
               Move::Right => assert_eq!(new_position, Position::new(3 + step, 3).unwrap()),
               Move::Left => assert_eq!(new_position, Position::new(3 - step, 3).unwrap()),
               Move::UpRight => assert_eq!(new_position, Position::new(3 + step, 3 + step).unwrap()),
               Move::UpLeft => assert_eq!(new_position, Position::new(3 - step, 3 + step).unwrap()),
               Move::DownRight => assert_eq!(new_position, Position::new(3 + step, 3 - step).unwrap()),
               Move::DownLeft => assert_eq!(new_position, Position::new(3 - step, 3 - step).unwrap()),
               _ => panic!("Invalid move for queen"),
           }
       }
   }
}

#[test]
fn test_king_moves() {
   let king = Piece::King(Color::White);
   let position = get_initial_position();

   let moveset = get_moveset(king);

   let expected_positions = vec![
       Position::new(3, 4).unwrap(),  // Up
       Position::new(3, 2).unwrap(),  // Down
       Position::new(4, 3).unwrap(),  // Right
       Position::new(2, 3).unwrap(),  // Left
       Position::new(4, 4).unwrap(),  // UpRight
       Position::new(2, 4).unwrap(),  // UpLeft
       Position::new(4, 2).unwrap(),  // DownRight
       Position::new(2, 2).unwrap(),  // DownLeft
   ];

   for (i, move_type) in moveset.moves.iter().enumerate() {
       let new_position = move_type.get_position(&position, 1);
       assert_eq!(new_position, expected_positions[i]);
   }
}