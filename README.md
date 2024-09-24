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

##### File
Enum for representing the vertical position

##### Rank
Enum for representing the horizontal position

#### Structs

##### Game
A representativ of the game, such as game state, the board, a turn indicator and more.

##### BoardPosition 
file: file
rank: rank
file and rank are enums. That range from `A..=H` and `1..=8`

##### Board
The board of the game. Holds the positions of all pieces. as well as the color of those pieces.

```Array[y][x];```

##### Moveset
* moves: Move - The direction of the moe
* steps: number - How many moves in that direction the piece can take
* colliding: bool - If the piece is stopped by other peices



##### Board
