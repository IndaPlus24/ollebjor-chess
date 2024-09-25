use olle_chess::*;
use board_position::*;

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
fn board_position_from_rank_and_file() {
    let bp1 = BoardPosition::new(Rank::D, File::Eight);
    let bp2 = BoardPosition::from_usize(3, 7);
    println!("{:?},{:?}", bp1, bp2);
    assert_eq!(bp1,bp2);

}

#[test]
fn move_set_is_some() {
    let game = Game::new();

    let bp1 = BoardPosition::new(Rank::B, File::Seven);

    let moves = game.get_possible_moves(&bp1);

    if let Some(m) = moves {
        println!("{m:?}");
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn pawn_can_move() {
    let mut game = Game::new();

    let bp1 = BoardPosition::new(Rank::C, File::Seven);
    let bp2 = BoardPosition::from_usize(2, 5);

    let pawn = game.board.get_piece_ref(&bp1).unwrap();
    
    let result = game.move_piece(&bp1, &bp2);
    if let Err(x) = result {
        println!("{x:?}");
    }

    assert_eq!(pawn, game.board.get_piece_ref(&bp2).unwrap());
}