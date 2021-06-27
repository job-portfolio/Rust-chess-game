use std::{io::{stdin, BufRead}, usize};

use crate::colour::Colour;

// check: Do the characters entered represent valid tiles on the chessboard?
fn is_valid_character(character: char) -> bool {
    if character < 'A' || character > 'H' {
        false
    } else {
        true
    }
}

// check: Do the numbers entered represent valid tiles on the chessboard?
fn is_valid_number(number: char) -> bool {
    if number < '1' || number > '8' {
        false
    } else {
        true
    }
}

// convert the users input 'A1B2' into indices that can be referenced in a 2D Array: A1B2 > 0,0,1,1
fn convertor(characters: &Vec<char>) -> Vec<usize> {
    
    let mut new_characters: Vec<usize> = Vec::new();

    for character in characters {
        match character {
            'A' => new_characters.push(0),
            'B' => new_characters.push(1),
            'C' => new_characters.push(2),
            'D' => new_characters.push(3),
            'E' => new_characters.push(4),
            'F' => new_characters.push(5),
            'G' => new_characters.push(6),
            'H' => new_characters.push(7),
    
            '1' => new_characters.push(0),
            '2' => new_characters.push(1),
            '3' => new_characters.push(2),
            '4' => new_characters.push(3),
            '5' => new_characters.push(4),
            '6' => new_characters.push(5),
            '7' => new_characters.push(6),
            '8' => new_characters.push(7),
            _ => new_characters.push(0),
        };
    }
    new_characters
}

pub fn read_user_input(player_colour: &Colour) -> Vec<usize> {   
    
    let mut user_move: Vec<usize> = Vec::new();

    let mut valid_move = false;
    
    let stdin = stdin();
    
    while !valid_move {

        let mut input = String::new();

        println!("| {:?} | Make your move: ", player_colour);

        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.len() != 4 {
            println!("Invalid move: Input must be of the form A1B2");
            continue;
        } else {
            let characters: Vec<char> = input.to_uppercase().chars().collect();
        
            if is_valid_character(characters[0]) && is_valid_number(characters[1]) && is_valid_character(characters[2]) && is_valid_number(characters[3]){
    
                user_move = convertor(&characters);
    
                valid_move = true;
            }
        }
    }
    user_move
}
