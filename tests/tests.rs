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

    println!("{:?}", game);

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
    let game = Game::new();

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
    let game = Game::new();

    let bp1 = BoardPosition::new(File::B, Rank::Seven);
    let bp2 = BoardPosition::from(Position::new(1, 5).unwrap());

    let moves = game.get_possible_moves(&bp1).unwrap_or(vec![]);
    println!("pawn move from {:?} to {:?}", bp1, bp2);
    assert!(moves == vec![bp2])
}

#[test]
fn test_moveset_for_white_pawn() {
    let game = setup_empty_at_e5(Piece::Pawn(Color::White));

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