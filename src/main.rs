use std::io;

// With how rust builds crates, a lib.rs is named
// after the parent directory being "closeTheBox"
// however this upsets the compiler as it prefers snake case
// for everything, which my repo is not called.
// so I declared a module game_box and made a gamebox.rs file
mod game_box;
mod single_player;
mod versus;

use crate::single_player::single_player;
use crate::versus::versus;

fn main() {
    println!( "Welcome to Close the Box!" );
    println!( "The objective is to move all tiles to the inactive position." );
    println!( "To move a tile to inactive, pick 1, 2, or 3 tiles that add to the sum of the dice roll." );
    println!( "Enter the tiles you want to select by typing in the numbers separated by a space and then hit 'Enter'." );
    println!( "i.e. 1 2 3" );
    println!( "Good luck!" );
    println!( "" );
    
    // Get player input
    let menu_select: usize;
    loop{
        println!( "Main Menu" );
        println!( "1) Single Player" );
        println!( "2) Versus" );
        let mut menu_select_string = String::new();
        io::stdin().read_line( &mut menu_select_string ).expect( "Failed to read line" );
        menu_select = match menu_select_string.trim().parse() {
            Ok( menu_select ) => {
                if menu_select == 1 || menu_select == 2 {
                    menu_select
                }
                else {
                    println!( "{menu_select} is not a valid input!" );
                    continue;
                }
                
            },
            Err( _ ) => {
                println!( "Was not a number!" );
                continue;
            }
        };

        break;
    }
    
    match menu_select {
        1 => single_player(),
        2 => versus(),
        _ => ()
    }
    

    println!( "" );
    println!( "Press 'Enter' to exit" );
    let mut pause = String::new();
    io::stdin().read_line( &mut pause ).expect( "Failed to read line" );
}
