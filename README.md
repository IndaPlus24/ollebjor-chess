# ollebjor-chess
Hello and welcome to Olles tjack engine!
### 1. The Game struct
The Game struct should be the only struct you have to use. Create and initialize a new game with `let game = Game::new();`. The `new` constructor calls the `init()` method automatically so all the pieces are setup correctly. Nice! 

_Side note:_ to create a game with an empty board, call the `Game::empty()` constructor. 

### 2. Moving the pieces
To move a piece, call the method `game.move_piece(from, to);` on your newly created game.

#### 2.1 BoardPosition
The `BoardPosition` struct is used to represent board positions (shocking). It is constructed like so: `BoardPosition::new(File::D, Rank::Four)`.
This example represents the position `D4` on the game's board.

#### 2.2 The File & Rank enums

![Board notation representation](image.png)
The picture shows the name of every position on the board. Use the enums `File` and `Rank` to create or find a position.
* `File` ranges from `A..=H` (these are the columns)
* `Rank` ranges from `1..=8` (these are the rows)

### 3. Game state och sånt
`game.move_piece(from, to) -> Result<GameState, ChessError>` returns a result that is either a `ChessError` enum (se the docs for explanation of each variant) or a GameState. 

### 4. Reacting to the game state
You can always read the current game state with `game.state`. No method will return a `GameState` enum without first changing the internal game state.

### 5. Example

Using what we've learnt from this <span style="color:orange">*AMAZING*</span> tutorial, we can now create a game!

```rust

let game: Game = Game::new();

if let Ok(new_state) = game.move_piece(BoardPosition::new(File::C, Rank::Two),BoardPosition::new(File::C, Rank::Three)){
   if let GameState::Promotion(position) = new_state {
      /*...
      do stuff to figure out the promoted_piece
      ...*/
      let result = game.promote_piece(promoted_piece);
      /*
      do stuff to handle the result, if the promotion fails, the game state is still GameState::Promotion(BoardPosition)
      */
   }
} else {
   println!("error!");
}

```
### Features
* [x] Turn indicator 👁️
* [x] Promotion ⬆️
* [x] Winning 👑
* [x] Check 🦧
* [x] Automatic board setup 🏁
* [x] Getting possible moves for piece 🕹️
* [x] Great method naming 🪧
* [x] Simply the most effective everything 🤓

### Coming soon...
* [ ] Pawn complete moveset (it cannot attack to sides, but from front) ⚔️
* [ ] Castling 🏰
* [ ] Stalemate 🪨
* [ ] Checkmate 🐐

Olle Björk 2024-09-27