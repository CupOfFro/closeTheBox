use std::io;
use rand::Rng;
// use std::cmp::Ordering;

// With how rust builds crates, a lib.rs is named
// after the parent directory being "closeTheBox"
// however this upsets the compiler as it prefers snake case
// for everything, which my repo is not called.
// so I declared a module game_box and made a gamebox.rs file
mod game_box;
use crate::game_box::{ GameBox, Tile };

fn main() {
    

    println!( "Welcome to Close the Box!" );
    println!( "The objective is to move all tiles to the inactive position." );
    println!( "To move a tile to inactive, pick 1, 2, or 3 tiles that add to the sum of the dice roll." );
    println!( "Enter the tiles you want to select by typing in the numbers separated by a space and then hit 'Enter'." );
    println!( "i.e. 1 2 3" );
    println!( "Good luck!" );
    println!( "" );

    let game_board_size: usize;
    loop {
        println!( "Enter game board size 9(default) - 12" );
        let mut game_board_size_input = String::new();
        io::stdin().read_line( &mut game_board_size_input ).expect( "Failed to read line" );
        game_board_size = match game_board_size_input.trim().parse() {
            Ok( game_board_size ) => {
                game_board_size
            }
            Err( _ ) => {
                println!( "Did not get a number!" );
                continue;
            }
        };
        break;
    }

    // Make the box
    let mut game_box: GameBox = GameBox::init( game_board_size );

    loop {
        if game_box.get_number_of_active_tiles() == 0 {
            println!( "YOU WIN!" );
            break;
        }

        let mut amount_of_dice = 2;

        if  game_box.tiles[ 8 ] == Tile::Inactive && game_box.tiles[ 7 ] == Tile::Inactive && game_box.tiles[ 6 ] == Tile::Inactive {

            loop {
                println!( "" );
                println!( "You got rid of 7, 8, and 9! You can now roll 1 or 2 dice!" );
                game_box.print_active_tiles();
                game_box.print_inactive_tiles();

                let mut amount_of_dice_string = String::new();
                io::stdin().read_line( &mut amount_of_dice_string ).expect( "Failed to read line" );
                amount_of_dice = match amount_of_dice_string.trim().parse()
                {
                    Ok( amount_of_dice ) => {
                        amount_of_dice
                    }
                    Err(_) => {
                        println!( "{amount_of_dice} was not a valid number! Enter 1 or 2." );
                        continue;
                    }
                };
                if amount_of_dice == 1 || amount_of_dice == 2 {
                    break;
                }
                else {
                    println!( "Value must be 1 or 2!" );
                }
            }
        }

        let d1: usize;
        let d2: usize;
        let dice_total: usize;

        match amount_of_dice {
            2 => {
                d1 = rand::thread_rng().gen_range( 1..=6 );
                d2 = rand::thread_rng().gen_range( 1..=6 );
                dice_total = d1 + d2;
            }
            _ => {
                d1 = rand::thread_rng().gen_range( 1..=6 );
                d2 = 0;
                dice_total = d1 + d2;
            }
        }
        match game_box.check_if_possible( &dice_total )  {
            0 => {
                println!( "" );
                game_box.print_active_tiles();
                game_box.print_inactive_tiles();
                println!( "(d1: {d1}) + (d2: {d2}) = (sum: {dice_total})" );
                println!( "No available moves!" );
                println!{ "You lose!" };
                break
            }
            _ => ()
        }

        'player_entry : loop {
            println!( "" );
            game_box.print_active_tiles();
            game_box.print_inactive_tiles();
            println!( "(d1: {d1}) + (d2: {d2}) = (sum: {dice_total})" );
            println!( "Enter up to 3 numbers that add up to the dice sum {dice_total}." );
            // Get player input
            // Create a mut String
            let mut selections = String::new();
            // Use thestd library to read into that string
            io::stdin().read_line( &mut selections ).expect( "Failed to read line" );
            // Print it
            // println!( "{}", selections );

            // Split the string up
            let selections = selections.split_whitespace();
            let mut number_of_entries = 0;
            let mut player_total: usize = 0;
            // For storing and comparing dice entries for duplicates
            let mut player_entries: [ usize; 4 ] = [ 0; 4 ];
            let mut input_index = 0;
            // now we can iterate through it
            for num in selections {
                number_of_entries += 1;
                if number_of_entries > 3 {
                    println!( "Too many values entered! Must be 4 or less!" );
                    break;
                }
                let num: usize = match num.trim().parse()
                {
                    Ok( num ) => {
                        // println!( "{num}" );
                        player_entries[ input_index ] = num;
                        input_index += 1;
                        num
                    }
                    Err(_) => {
                        println!( "{num} was not a valid number!" );
                        break;
                    }
                };
                player_total += num;
            }

            // Check for duplicates
            for ( i, val ) in player_entries.iter().enumerate() {
                for j in i+1..4
                {
                    if *val == player_entries[ j ] {
                        if *val == 0 || player_entries[ j ] == 0 {
                            continue;
                        }
                        println!( "duplicate detected! {val} and {}", player_entries[j] );
                        break 'player_entry;
                    }
                }
            }
            
            if player_total == dice_total {
                for val in player_entries {
                    // println!( "{val}" );
                    if val == 0 {
                        continue;
                    }
                    // Arrays are 0 based, but our tiles are 1 based
                    // so we need to subtract 1 from what tile the player
                    // entered to "flip" it
                    game_box.tiles[ val - 1 ] = Tile::Inactive;
                }
                break;
            }
            else {
                println!( "Entry did not add up to dice sum!" );
            }
        }
    }

    println!( "" );
    println!( "Press 'Enter' to exit" );
    let mut pause = String::new();
    io::stdin().read_line( &mut pause ).expect( "Failed to read line" );
}
