# Rust-chess-game
A chess game built in Rust

A portfolio project incorporating readability, reusability, encapsulation, commenting, naming conventions and memory utilisation. The visibility of functions have been considered.

The code is divided into six Rust files:
  - Chessboard: Handles all the logic for the board and individual tiles
  - Piece: specifies the different chess pieces
  - Colour: specifies the player colours (white and black)
  - User_input: Validates the input of the user
  - Movement: Validates the chess move
  - Main: Starts the game, displays the board and assesses victory conditions

The board:

 | A| B| C| D| E| F| G| H|
===========================
1|wR wN wB wQ wK wB wN wR |
2|wP wP wP wP wP wP wP wP |
3|                        |
4|                        |
5|                        |
6|                        |
7|bP bP bP bP bP bP bP bP |
8|bR bN bB bQ bK bB bN bR |
===========================

Code Walkthrough:
  - Each player is created, denoted by their colour: white | black
  - The victory condition is set to false. The game keeps looping through player turns until one king is destroyed and victory condition is set to true.
  - The board is created
  - The board is printed to the screen (seen above)
  - Execution is passed to movement.rs to validate users input and validate the chess move
    - valid_move is set to false to enable the user to re-enter their move should either the user_input or chess move be invalid.
      -user input is validated
      -chess move is validated
        -the validation function used depends on the result of a call to get_piece(), this determines the chess piece chosen by the player
          -when checkmate is arrived at, the board is wiped and each tile is set to empty.
  - Once the white player has completed a valid move, execution is passed back to main
  - Main checks the victory condition (whether the board has been wiped)
    - If victory condition is true, the winning player is printed to the screen and the game ends.
    - If victory condition is false, the other players turn begins

Contents:
  - chessboard.rs
    - STRUCT: Chessboard
    - IMPL: Chessboard
      -FUNCTION: start_game()
      -FUNCTION: draw_board()
      -FUNCTION: check_victory()
    - ENUM: Tile
    - IMPL: Tile
      -FUNCTION: tile_state()
      -FUNCTION: get_piece()
      -FUNCTION: get_colour()
  - colour.rs
    - ENUM: Colour
  - main.rs
  - movement.rs
    - FUNCTION: make_move()
    - FUNCTION: convertor()
    - FUNCTION: validate_pawn_move()
    - FUNCTION: validate_rook_move()
    - FUNCTION: validate_knight_move()
    - FUNCTION: validate_bishop_move()
    - FUNCTION: validate_king_move()
    - FUNCTION: validate_queen_move()
  - piece.rs
    - ENUM: Piece
  - user_input.rs
    - FUNCTION: read_user_input()
    - FUNCTION: convertor()
    - FUNCTION: is_valid_character()
    - FUNCTION: is_valid_number()


The program can be run by using cargo build and cargo run while inside the chess-game folder.
