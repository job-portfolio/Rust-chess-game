use crate::piece::Piece;
use crate::colour::Colour;

#[derive(Debug)]
pub struct Chessboard {
    pub board: [[Tile; 8]; 8],
}

impl Chessboard {
    pub fn start_game() -> [[Tile;8]; 8] {
    
        println!("\nEXAMPLE: To move from A1 to B2, please type: A1B2");
        println!("================ Enjoy your game ================\n");
    
        let game = Chessboard {
            board: [
                [
                    Tile::Occupied(Piece::Rook,     Colour::White),
                    Tile::Occupied(Piece::Knight,   Colour::White),
                    Tile::Occupied(Piece::Bishop, 	Colour::White),
                    Tile::Occupied(Piece::Queen, 	Colour::White),
                    Tile::Occupied(Piece::King, 	Colour::White),
                    Tile::Occupied(Piece::Bishop, 	Colour::White),
                    Tile::Occupied(Piece::Knight, 	Colour::White),
                    Tile::Occupied(Piece::Rook,	    Colour::White),
                ],
                [Tile::Occupied(Piece::Pawn, Colour::White); 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Occupied(Piece::Pawn, Colour::Black); 8],
                [
                    Tile::Occupied(Piece::Rook,     Colour::Black),
                    Tile::Occupied(Piece::Knight,   Colour::Black),
                    Tile::Occupied(Piece::Bishop, 	Colour::Black),
                    Tile::Occupied(Piece::Queen, 	Colour::Black),
                    Tile::Occupied(Piece::King, 	Colour::Black),
                    Tile::Occupied(Piece::Bishop, 	Colour::Black),
                    Tile::Occupied(Piece::Knight, 	Colour::Black),
                    Tile::Occupied(Piece::Rook,	    Colour::Black),
                ],
            ],
        };
        game.board
    }

    pub fn draw_board(board : &[[Tile;8]; 8]) {
    
        let mut index = 0;
    
        println!(" | A| B| C| D| E| F| G| H|");
        println!("===========================");
        
        for row in board.iter() {
            index += 1;
            print!("{:?}|", index);
            for tile in row {
                //print!("{} ", tile_state(*tile));
                print!("{} ", tile.tile_state());
            }
            print!("|\n");
        }
        println!("===========================\n");
    }
    
    pub fn check_victory(board : &[[Tile;8]; 8]) -> bool {
        let mut all_tiles_empty = true;
    
        while all_tiles_empty{
            for row in board {
                for tile in row {
                    if tile != &Tile::Empty {
                        all_tiles_empty = false
                    }
                }
            }
        }
        all_tiles_empty
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Occupied(Piece, Colour),
}

impl Tile {
    fn tile_state(&self) -> &'static str {
        match self {
            Tile::Occupied(Piece::Pawn,     Colour::White) => "wP",
            Tile::Occupied(Piece::Rook,	    Colour::White) => "wR",
            Tile::Occupied(Piece::Knight,	Colour::White) => "wN",
            Tile::Occupied(Piece::Bishop, 	Colour::White) => "wB",
            Tile::Occupied(Piece::Queen, 	Colour::White) => "wQ",
            Tile::Occupied(Piece::King, 	Colour::White) => "wK",
            
            Tile::Occupied(Piece::Pawn,	    Colour::Black) => "bP",
            Tile::Occupied(Piece::Rook,	    Colour::Black) => "bR",
            Tile::Occupied(Piece::Knight,	Colour::Black) => "bN",
            Tile::Occupied(Piece::Bishop, 	Colour::Black) => "bB",
            Tile::Occupied(Piece::Queen, 	Colour::Black) => "bQ",
            Tile::Occupied(Piece::King, 	Colour::Black) => "bK",
    
            _ => "  ",
        }
    }

    pub fn get_colour(&self) -> Colour {
        match self {
            Tile::Occupied(Piece::Pawn,     Colour::White) => Colour::White,
            Tile::Occupied(Piece::Rook,	    Colour::White) => Colour::White,
            Tile::Occupied(Piece::Knight,	Colour::White) => Colour::White,
            Tile::Occupied(Piece::Bishop, 	Colour::White) => Colour::White,
            Tile::Occupied(Piece::Queen, 	Colour::White) => Colour::White,
            Tile::Occupied(Piece::King, 	Colour::White) => Colour::White,
            
            Tile::Occupied(Piece::Pawn,	    Colour::Black) => Colour::Black,
            Tile::Occupied(Piece::Rook,	    Colour::Black) => Colour::Black,
            Tile::Occupied(Piece::Knight,	Colour::Black) => Colour::Black,
            Tile::Occupied(Piece::Bishop, 	Colour::Black) => Colour::Black,
            Tile::Occupied(Piece::Queen, 	Colour::Black) => Colour::Black,
            Tile::Occupied(Piece::King, 	Colour::Black) => Colour::Black,
    
            _ => Colour::Empty,
        }
    }
    
    pub fn get_piece(&self) -> Piece {
        match self {
            Tile::Occupied(Piece::Pawn,     Colour::White) => Piece::Pawn,
            Tile::Occupied(Piece::Rook,	    Colour::White) => Piece::Rook,
            Tile::Occupied(Piece::Knight,	Colour::White) => Piece::Knight,
            Tile::Occupied(Piece::Bishop, 	Colour::White) => Piece::Bishop,
            Tile::Occupied(Piece::Queen, 	Colour::White) => Piece::Queen,
            Tile::Occupied(Piece::King, 	Colour::White) => Piece::King,
            
            Tile::Occupied(Piece::Pawn,	    Colour::Black) => Piece::Pawn,
            Tile::Occupied(Piece::Rook,	    Colour::Black) => Piece::Rook,
            Tile::Occupied(Piece::Knight,	Colour::Black) => Piece::Knight,
            Tile::Occupied(Piece::Bishop, 	Colour::Black) => Piece::Bishop,
            Tile::Occupied(Piece::Queen, 	Colour::Black) => Piece::Queen,
            Tile::Occupied(Piece::King, 	Colour::Black) => Piece::King,
    
            _ => Piece::Pawn,
        }
    }
}