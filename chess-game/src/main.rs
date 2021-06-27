mod chessboard;
mod piece;
mod colour;
mod movement;
mod user_input;

use chessboard::Chessboard;
use colour::Colour;

pub fn main() {
    
    let players = [Colour::White, Colour::Black];

    let mut victory = false;

    let mut board = Chessboard::start_game();
    
    while !victory {

        for player_colour in players {

            Chessboard::draw_board(&board);

            board = movement::make_move(board, &player_colour);
        
            victory = Chessboard::check_victory(&board);
        }
    }
}