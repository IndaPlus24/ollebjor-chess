# ollebjor-chess
Tjack

## Docs
#### Enums
##### Piece
* Pawn
* Knight
* King
* Queen
* Bishop
* Rook
Piece kan innehålla en color för att visa vilken färg det är.

##### Color
* Black
* White

#### Structs

##### Game
A representativ of the game, such as game state, the board, a turn indicator and more.

##### Position 
file: file
rank: rank
file and rank are enums. That range from `A..=H` and `1..=8`

##### Board
The board of the game. Holds the positions of all pieces. as well as the color of those pieces.

Hashmap[(file, rank)] = Piece(Color)

##### Board
##### Board
