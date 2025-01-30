use std::io;
use rand::Rng;
use crate::game_box::{ GameBox, Tile };

pub fn single_player() {
    // Make the box
    let mut game_box: GameBox = GameBox::init();

    loop {
        if game_box.get_number_of_tiles( Tile::Active ) == 0 {
            println!( "YOU WIN!" );
            break;
        }

        let mut amount_of_dice = 2;

        if  game_box.check_if_single_die_possible( Tile::Active ) {

            loop {
                println!( "" );
                println!( "You got rid of all tiles above 6! You can now roll 1 or 2 dice!" );
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
        match game_box.check_if_possible( &dice_total, Tile::Inactive )  {
            false => {
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

        game_box.player_entry( d1, d2, Tile::Active );
    }
}