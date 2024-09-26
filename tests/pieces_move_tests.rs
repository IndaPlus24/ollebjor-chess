use olle_chess::*;
// use olle_chess::moveset::*;
// use olle_chess::board::*;
use olle_chess::position::*;

fn get_initial_position() -> Position {
    Position::new(3, 3).unwrap() // Sample position at D4
}

fn print_legal_moves(piece: Piece, position: &BoardPosition) {
    let mut game = Game::empty();

    println!("\nLegal moves for {:?} at position {:?}:", piece, position);

    // Placera pjäsen på startpositionen
    game.board.spawn_piece(piece, &position.into()).unwrap();
    println!("{}", game.board);

    // Gå igenom varje drag i movesetet och räkna upp möjliga positioner
    if let Some(moves) = game.get_possible_moves(position) {
        for bp in moves {
            println!("Move {:?} -> {:?}", position, bp);
        }
    }

    println!("\n--------------------------\n");
}

#[test]
fn test_legal_moves_visualization() {
    let position = get_initial_position();

    // Test för varje typ av pjäs
    let pieces = vec![
        Piece::Pawn(Color::White),
        Piece::Rook(Color::White),
        Piece::Knight(Color::White),
        Piece::Bishop(Color::White),
        Piece::Queen(Color::White),
        Piece::King(Color::White),
    ];

    for piece in pieces {
        print_legal_moves(piece, &position.into());
    }
}
