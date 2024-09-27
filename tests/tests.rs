use std::vec;

use olle_chess::*;
use position::*;

// Detta är mina egna tests som är lite sämre....

fn setup_empty_at_e5(piece: Piece) -> Game {
    let mut game = Game::empty();
    game.board.spawn_piece(piece, &BoardPosition::from("E5").into()).expect("could not spawn piece!");
    return  game;
}

// check test framework
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// example test
// check that game state is in progress after initialisation
#[test]
fn game_in_progress_after_init() {
    let game = Game::new();
    assert_eq!(game.get_game_state(), GameState::InProgress);
}

#[test]
fn white_is_first() {
    let game = Game::new();
    assert_eq!(game.turn, Color::White);
}

#[test]
fn board_position_from_rank_and_file_equals_board_position_from_position() {
    let bp1 = BoardPosition::new( File::B, Rank::Seven);
    let bp2 = BoardPosition::from(Position::new(1, 6).unwrap());
    println!("{:?},{:?}", bp1, bp2);
    assert_eq!(bp1,bp2);
}

#[test]
fn move_set_is_some() {
    let mut game = Game::new();
    let bp1 = BoardPosition::new(File::B, Rank::Seven);
    let moves = game.get_possible_moves(&bp1);

    if let Some(m) = moves {
        println!("{m:?}");
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn test_moveset_for_pawn_works_is_right() {
    let mut game = Game::empty();
    game.board.spawn_piece(Piece::Pawn(Color::White), &BoardPosition::new(File::B, Rank::Two).into()).unwrap();

    let bp1 = BoardPosition::new(File::B, Rank::Two);
    let bp2 = BoardPosition::new(File::B, Rank::Three);

    println!("{:?}", game);
    if let Some(moves) = game.get_possible_moves(&bp1){
        println!("pawn move from {:?} to {:?}", bp1, bp2);
        println!("found legal moves: {:?}", moves);
        println!("actual legal moves: {:?}", vec![bp2]);
        
        assert_eq!(moves, vec![bp2]);
    }
}

#[test]
fn test_moveset_for_white_pawn() {
    let mut game = setup_empty_at_e5(Piece::Pawn(Color::White));

    let bp1 = BoardPosition::new(File::E, Rank::Five);
    let bp2 = BoardPosition::new(File::E, Rank::Six);

    let moves = game.get_possible_moves(&bp1).unwrap_or(vec![]);
    println!("pawn move from {:?} to {:?}", bp1, bp2);
    assert!(moves == vec![bp2])
}

#[test]
fn board_is_facing_right_direction(){
    let game = Game::new();

    println!("{:?}", game.board);
    println!("{}", game.board);
    println!("{:?}", game);
}

#[test]
fn test_piece_actually_moves() {
    let mut game = setup_empty_at_e5(Piece::Pawn(Color::White));

    let bp1 = BoardPosition::new(File::E, Rank::Five);
    let bp2 = BoardPosition::new(File::E, Rank::Six);

    println!("Before:\n{:?}", game);
    game.move_piece(&bp1, &bp2).expect("could not move piece!");
    println!("After:\n{:?}", game);

    assert_eq!(game.board.get_piece(&bp1.into()), None);
    assert_eq!(game.board.get_piece(&bp2.into()), Some(Piece::Pawn(Color::White)));
}

#[test]
fn test_is_check_works() {
    //Setup empty board with rook and kings
    let mut game = Game::empty();
    game.board.spawn_piece(Piece::King(Color::White), &BoardPosition::new(File::E, Rank::One).into()).unwrap();
    game.board.spawn_piece(Piece::King(Color::Black), &BoardPosition::new(File::E, Rank::Eight).into()).unwrap();
    game.board.spawn_piece(Piece::Rook(Color::Black), &BoardPosition::new(File::H, Rank::One).into()).unwrap();

    //Check if white king is in check
    println!("{:?}", game);

    //Shouold error
    let result = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::D, Rank::One));
    print!("Should error: {:?}", result);
    assert!(result.is_err());

    println!("{:?}", game);

    println!("Should not error{:?}", result);
    let result = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::D, Rank::Two));
    assert!(result.is_ok());

    println!("{:?}", game);
}

#[test]
fn test_turn_is_switched_after_move() {
    //Setup empty board with rook and kings
    let mut game = Game::empty();
    game.board.spawn_piece(Piece::King(Color::White), &BoardPosition::new(File::E, Rank::One).into()).unwrap();
    game.board.spawn_piece(Piece::King(Color::Black), &BoardPosition::new(File::E, Rank::Eight).into()).unwrap();
    game.board.spawn_piece(Piece::Rook(Color::Black), &BoardPosition::new(File::H, Rank::One).into()).unwrap();
    
    println!("{:?}", game);

    //Move King to D1 -> error -> turn should not change
    let result = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::D, Rank::One));
    assert!(result.is_err());
    assert_eq!(game.turn, Color::White);
    println!("{:?}", game);

    //Move King to D2 -> ok -> turn should change
    let result = game.move_piece(&BoardPosition::new(File::E, Rank::One), &BoardPosition::new(File::D, Rank::Two));
    assert!(result.is_ok());
    assert_eq!(game.turn, Color::Black);
    println!("{:?}", game);
}

#[test]
fn test_promotion_works() {
    let mut game = Game::empty();
    game.board.spawn_piece(Piece::Pawn(Color::White), &BoardPosition::new(File::A, Rank::Seven).into()).unwrap();
    game.board.spawn_piece(Piece::King(Color::White), &BoardPosition::new(File::H, Rank::One).into()).unwrap();
    game.board.spawn_piece(Piece::King(Color::Black), &BoardPosition::new(File::H, Rank::Eight).into()).unwrap();

    println!("{:?}", game);

    let result = game.move_piece(&BoardPosition::new(File::A, Rank::Seven), &BoardPosition::new(File::A, Rank::Eight));
    assert!(result.is_ok());
    println!("{:?}", game);
    assert_eq!(game.get_game_state(), GameState::Promotion(BoardPosition::new(File::A, Rank::Eight)));    

    //Try to move black king -> error
    let result = game.move_piece(&BoardPosition::new(File::H, Rank::Eight), &BoardPosition::new(File::H, Rank::Seven));
    assert!(result.is_err());
    //Turn has changed but white still needs to promote
    assert_eq!(game.turn, Color::Black);
    assert_eq!(game.get_game_state(), GameState::Promotion(BoardPosition::new(File::A, Rank::Eight)));

    //Promote to Queen
    let result = game.promote_pawn(Piece::Queen(Color::White));
    assert!(result.is_ok());
    assert_eq!(game.get_game_state(), GameState::Check);

    println!("{:?}", game);

    //Move black king -> ok
    let result = game.move_piece(&BoardPosition::new(File::H, Rank::Eight), &BoardPosition::new(File::H, Rank::Seven));
    assert!(result.is_ok());
}