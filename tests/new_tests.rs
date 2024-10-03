use std::result;

use olle_chess::*;
use position::*;

#[cfg(test)]
// Support functions to help setup test scenariaos
fn setup_empty_with_kings() -> Game {
   let mut game = Game::empty();
   game.board.spawn_piece(Piece::King(Color::White), &BoardPosition::new(File::E, Rank::One).into()).unwrap();
   game.board.spawn_piece(Piece::King(Color::Black), &BoardPosition::new(File::E, Rank::Eight).into()).unwrap();
   return game;
}

#[test]
fn test_turn_starts_as_white() {
    let game = Game::empty();
    assert_eq!(game.get_turn(), Color::White);
}

#[test]
fn test_game_is_won_by_white() {
   let mut game = Game::empty();
   game.board.spawn_piece(Piece::King(Color::White), &BoardPosition::new(File::E, Rank::One).into()).unwrap();

   let _ = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::E, Rank::Two));
   
   assert_eq!(game.get_game_state(), GameState::GameOver(Color::White));
}

#[test]
fn test_game_is_won_by_black() {
   let mut game = Game::empty();
   game.board.spawn_piece(Piece::King(Color::Black), &BoardPosition::new(File::E, Rank::Eight).into()).unwrap();
   game.board.spawn_piece(Piece::Pawn(Color::White), &BoardPosition::new(File::A, Rank::Five).into()).unwrap();

   //Change turn to black
   let _ = game.move_piece(&BoardPosition::new(File::A, Rank::Five), &BoardPosition::new(File::A, Rank::Six));

   //Move black king to win the game
   let _ = game.move_piece(&BoardPosition::new(File::E, Rank::Eight), &BoardPosition::new(File::E, Rank::Seven));
   
   assert_eq!(game.get_game_state(), GameState::GameOver(Color::Black));
}

#[test]
fn test_pawn_can_move_straight_forward_one_step() {
   // White Pawn
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Pawn(Color::White), &BoardPosition::new(File::E, Rank::Two).into()).unwrap();
   game.board.spawn_piece(Piece::Pawn(Color::Black), &BoardPosition::new(File::E, Rank::Seven).into()).unwrap();

   let result = game.move_piece(&BoardPosition::new(File::E, Rank::Two), &BoardPosition::new(File::E, Rank::Three)); 
   println!("Result after white pawn move: {:?}", result);
   assert!(result.is_ok());

   // Black Pawn
   let result = game.move_piece(&BoardPosition::new(File::E, Rank::Seven), &BoardPosition::new(File::E, Rank::Six)); 
   println!("Result after black pawn move: {:?}", result);
   assert!(result.is_ok());
}

#[test]
fn test_pawn_can_be_promoted() {
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Pawn(Color::White), &BoardPosition::new(File::A, Rank::Seven).into()).unwrap();
   game.board.spawn_piece(Piece::Pawn(Color::Black), &BoardPosition::new(File::A, Rank::Two).into()).unwrap();
   
   //Testing white can be promoted
   let result = game.move_piece(&BoardPosition::new(File::A, Rank::Seven), &BoardPosition::new(File::A, Rank::Eight)); 
   println!("Result after white pawn move: {:?}", result);
   assert!(result.is_ok());
   assert_eq!(game.get_game_state(), GameState::Promotion(BoardPosition::new(File::A, Rank::Eight)));

   let result = game.promote_pawn(Piece::Bishop(Color::White));
   println!("Result after white pawn promotion: {:?}", result);
   assert!(result.is_ok());
   assert_eq!(game.get_game_state(), GameState::InProgress);

   //Testing black can be promoted
   let result = game.move_piece(&BoardPosition::new(File::A, Rank::Two), &BoardPosition::new(File::A, Rank::One)); 
   println!("Result after white pawn move: {:?}", result);
   assert!(result.is_ok());
   assert_eq!(game.get_game_state(), GameState::Promotion(BoardPosition::new(File::A, Rank::One)));

   let result = game.promote_pawn(Piece::Bishop(Color::Black));
   println!("Result after white pawn promotion: {:?}", result);
   assert!(result.is_ok());
   assert_eq!(game.get_game_state(), GameState::InProgress);
}

#[test]
fn test_black_king_in_check() {
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Rook(Color::White), &BoardPosition::new(File::H, Rank::Eight).into()).unwrap();
   game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::E, Rank::Two)).unwrap();
   
   println!("turn: {:?}", game.get_turn());
   println!("{:?}", game.get_game_state());
   println!("{:?}", game);
   assert_eq!(game.get_game_state(), GameState::Check);
}

#[test]
fn test_white_cannot_put_themselves_in_check() {
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Rook(Color::Black), &BoardPosition::new(File::H, Rank::One).into()).unwrap();
   
   let result = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::F, Rank::One));
   
   assert!(result.is_err());
   assert_eq!(result.err().unwrap(), ChessError::IllegalMove);
}

#[test]
fn test_black_cannot_put_themselves_in_check() {
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Rook(Color::White), &BoardPosition::new(File::H, Rank::Eight).into()).unwrap();
   //Move white king to change the turn
   game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::E, Rank::Two)).unwrap();

   let result = game.move_piece(&BoardPosition::new(File::E, Rank::Eight), &BoardPosition::new(File::F, Rank::Eight));

   assert!(result.is_err());
   assert_eq!(result.err().unwrap(), ChessError::IllegalMove);
}

#[test]
fn test_game_is_in_progress_when_moving_out_of_check() {
   let mut game = setup_empty_with_kings();
   game.board.spawn_piece(Piece::Rook(Color::White), &BoardPosition::new(File::H, Rank::Eight).into()).unwrap();
   //Move white to change the turn
   game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::E, Rank::Two)).unwrap();
   //Move the black king to go out of check
   game.move_piece(&BoardPosition::new(File::E, Rank::Eight), &BoardPosition::new(File::E, Rank::Seven)).unwrap();
   
   assert_eq!(game.get_game_state(), GameState::InProgress);
}