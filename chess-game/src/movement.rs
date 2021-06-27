use crate::chessboard::Tile;
use crate::colour::Colour;
use crate::piece::Piece;
use crate::user_input;

pub fn make_move(mut board: [[Tile; 8]; 8], player_colour: &Colour) -> [[Tile; 8]; 8] {

    let mut valid_move = false;

    while !valid_move {

        let user_move = user_input::read_user_input(&player_colour);

        let y_src = user_move[0];
        let x_src = user_move[1];
        let y_dest = user_move[2];
        let x_dest = user_move[3];

        let source_tile = board[x_src][y_src];

        // check move: Is the source tile empty?
        if source_tile == Tile::Empty {
            println!("Invalid Move: The tile you want to move from is empty");
            continue;
        }         

        let src_piece = source_tile.get_piece();
        let src_colour = source_tile.get_colour();

        // check move: Is the move acceptable for the chosen chess piece?
        let validated_move = match src_piece {
            Piece::Pawn => validate_pawn_move(&user_move, src_colour),
            Piece::Rook => validate_rook_move(&user_move),
            Piece::Knight => validate_knight_move(&user_move),
            Piece::Bishop => validate_bishop_move(&user_move),
            Piece::Queen => validate_queen_move(&user_move),
            Piece::King => validate_king_move(&user_move),
        };

        if !validated_move {
            continue;
        }
        
        // check move: Does the player control the piece on the source tile?
        if src_colour != *player_colour {
            println!("Invalid Move: The tile you want to move from does not contain a piece you control.");
            continue;
        }
        
        let destination_tile = board[x_dest][y_dest];

        // check move: Is the destination tile empty?
        if destination_tile == Tile::Empty {
            
            // move the piece
            board[x_src][y_src] = Tile::Empty;
            board[x_dest][y_dest] = Tile::Occupied(src_piece, src_colour);

            valid_move = true;
            continue;
        } else {

            let dest_piece = destination_tile.get_piece();
            let dest_colour = destination_tile.get_colour();

            // check move: Does the player control the piece on the destination tile? 
            if dest_colour == *player_colour {
                println!("Invalid Move: The tile you want to move to already has one of your pieces on it");
                continue;
            } else {

                if dest_piece == Piece::King {
                    println!("{:?} won game!", player_colour);
                    return [[Tile::Empty; 8]; 8];
                } else {
                    
                    // move the piece
                    board[x_src][y_src] = Tile::Empty;
                    board[x_dest][y_dest] = Tile::Occupied(src_piece, src_colour);

                    valid_move = true;
                    continue;
                } 
            }  
        }
    }
    board
}

// convert vector of usize elements into a vector of isize elements. This allows move calculations to have negative values
fn convertor(usize_move: &Vec<usize>) -> Vec<isize> {
    
    let mut isize_move: Vec<isize> = Vec::new();

    for coord in usize_move {
        match coord {    
            0 => isize_move.push(0),
            1 => isize_move.push(1),
            2 => isize_move.push(2),
            3 => isize_move.push(3),
            4 => isize_move.push(4),
            5 => isize_move.push(5),
            6 => isize_move.push(6),
            7 => isize_move.push(7),
            _ => isize_move.push(0),
        };
    }
    isize_move
}

fn validate_pawn_move(user_move: &Vec<usize>, src_colour: Colour) -> bool {
    
    let isize_move = convertor(&user_move);

    let y_src = isize_move[0];
    let x_src = isize_move[1];
    let y_dest = isize_move[2];
    let x_dest = isize_move[3];

    let x_move = x_src - x_dest;
    let y_move = y_src - y_dest;

    let mut validate_move = false;

    if src_colour == Colour::White {
        if x_move == -1 && y_move == 0 {
            validate_move = true;
        }
    } else if src_colour == Colour::Black {
        if x_move == 1 && y_move == 0 {
            validate_move = true;
        }
    } else {
        println!("Invalid move: Your Pawn cannot move like that.")
    }

    validate_move
}

fn validate_rook_move(user_move: &Vec<usize>) -> bool {
    
    let isize_move = convertor(&user_move);

    let y_src = isize_move[0];
    let x_src = isize_move[1];
    let y_dest = isize_move[2];
    let x_dest = isize_move[3];

    let x_move = x_src - x_dest;
    let y_move = y_src - y_dest;

    let mut validate_move = false;

    if (x_move == 0 && y_move > 0)
        || (x_move == 0 && y_move < 0)
        || (x_move > 0 && y_move == 0)
        || (x_move < 0 && y_move == 0) {

        validate_move = true;
    } else {
        println!("Invalid move: Your Rook cannot move like that.")
    }
    validate_move 
}

fn validate_knight_move(user_move: &Vec<usize>) -> bool {
    
    let isize_move = convertor(&user_move);

    let y_src = isize_move[0];
    let x_src = isize_move[1];
    let y_dest = isize_move[2];
    let x_dest = isize_move[3];

    let x_move = x_src - x_dest;
    let y_move = y_src - y_dest;

    let mut validate_move = false;

    if (x_move == 2 && y_move == 1)
        || (x_move == 2 && y_move == -1)
        || (x_move == -2 && y_move == 1)
        || (x_move == -2 && y_move == -1)
        || (x_move == 1 && y_move == 2)
        || (x_move == -1 && y_move == 2)
        || (x_move == 1 && y_move == -2)
        || (x_move == -1 && y_move == -2) {

        validate_move = true;
    } else {
        println!("Invalid move: Your Knight cannot move like that.")
    }
    validate_move 
}

fn validate_bishop_move(user_move: &Vec<usize>) -> bool {
    
    let isize_move = convertor(&user_move);

    let y_src = isize_move[0];
    let x_src = isize_move[1];
    let y_dest = isize_move[2];
    let x_dest = isize_move[3];

    let x_move = x_src - x_dest;
    let y_move = y_src - y_dest;

    let mut validate_move = false;

    if (x_move + y_move == 0)
        || (x_move + y_move == x_move * 2)
        || (x_move + y_move == y_move * 2) {

        validate_move = true;
    } else {
        println!("Invalid move: Your Bishop cannot move like that.")
    }
    validate_move 
}
fn validate_king_move(user_move: &Vec<usize>) -> bool {
    
    let isize_move = convertor(&user_move);

    let y_src = isize_move[0];
    let x_src = isize_move[1];
    let y_dest = isize_move[2];
    let x_dest = isize_move[3];

    let x_move = x_src - x_dest;
    let y_move = y_src - y_dest;

    let mut validate_move = false;

    if (x_move == 1 && y_move == 1)
        || (x_move == 1 && y_move == 0)
        || (x_move == 1 && y_move == -1)
        || (x_move == 0 && y_move == 1)
        || (x_move == 0 && y_move == -1)
        || (x_move == -1 && y_move == 1)
        || (x_move == -1 && y_move == 0)
        || (x_move == -1 && y_move == -1) {

        validate_move = true;
    } else {
        println!("Invalid move: Your King cannot move like that.")
    }
    validate_move 
}
fn validate_queen_move(user_move: &Vec<usize>) -> bool {
    
    let mut validate_move = false;
    
    if validate_rook_move(&user_move) || validate_bishop_move(&user_move){
        validate_move = true;
    } else {
        println!("Invalid move: Your Queen cannot move like that.")
    }

    validate_move
}