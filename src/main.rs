use std::io;
use rand::Rng;
// use std::cmp::Ordering;

/*
I need these two traits to use the enum as an array element
*/
#[derive(Copy, Clone, PartialEq)]
// Below is my Tile enum. A tile can be active or inactive
enum Tile {
    Active,
    Inactive
}

// My struct for the gameboard. It holds 9 Tiles
struct GameBox {
    tiles: [ Tile; 9 ]
}

impl GameBox {
    // Create a new gameboard and return self
    fn init() -> Self {
        Self {
            tiles: [ Tile::Active; 9 ]
        }
    }

    fn print_active_tiles( &self ) {
        print!( "  Active Tiles:" );
        for (i, t) in self.tiles.iter().enumerate() {
            match t {
                Tile::Active =>{
                    let tile_index = i + 1;
                    print!( " {tile_index}" )
                }
                // Tile::Inactive => ()
                _ => ()
            }
        }
        println!( "" );
    }

    fn print_inactive_tiles( &self ) {
        print!( "Inactive Tiles:" );
        for ( i, t ) in self.tiles.iter().enumerate() {
            match t {
                Tile::Inactive => {
                    let tile_index = i + 1;
                    print!( " {tile_index}" );
                }
                // This is short hand for match anything else and do nothing
                _ => ()
            }
        }
        println!( "" );
    }

    fn get_number_of_active_tiles( &self ) -> i32 {
        let mut count = 0;
        for i in self.tiles {
            if i == Tile::Active {
                count += 1;
            }
        }
        count
    }

    // This function will take the dice roll, and see if any 1, 2, 3, or 4
    // combinations of tiles can add to the dice sum
    fn check_if_possible( &self, dice_roll: &usize ) -> u8 {
        // println!( "Checking for {dice_roll}" );
        for i in 0..self.tiles.len() {
            if self.tiles[ i ] == Tile::Inactive {
                continue;
            }
            if i + 1 == *dice_roll {
                // println!( "{} matches!", i+1 );
                return 1
            }
            for j in i+1..self.tiles.len() {
                if self.tiles[ j ] == Tile::Inactive {
                    continue;
                }
                if i + 1 + j + 1 == *dice_roll {
                    // println!( "{} {} matches!", i+1, j+1 );
                    return 1
                }
                for k in j+1..self.tiles.len() {
                    if self.tiles[ k ] == Tile::Inactive {
                        continue;
                    }
                    if i + 1 + j + 1 + k + 1 == *dice_roll {
                        // println!( "{} {} {} matches!", i+1, j+1, k+1 );
                        return 1
                    }
                    for l in k+1..self.tiles.len() {
                        if self.tiles[ l ] == Tile::Inactive {
                            continue;
                        }
                        if i + 1 + j + 1 + k + 1 + l + 1 == *dice_roll {
                            // println!( "{} {} {} {} matches!", i+1, j+1, k+1, l+1 );
                            return 1
                        }
                    }
                }
            }
        }
        0
    }
}

fn main() {
    // Make the box
    let mut game_box: GameBox = GameBox::init();

    loop {
        if game_box.get_number_of_active_tiles() == 0 {
            println!( "YOU WIN!" );
            break;
        }

        let mut amount_of_dice = 2;

        if  game_box.tiles[ 8 ] == Tile::Inactive && game_box.tiles[ 7 ] == Tile::Inactive && game_box.tiles[ 6 ] == Tile::Inactive {

            loop {
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
                break;
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
                println!( "d1: {d1} d2: {d2} sum: {dice_total}" );
                println!{ "You lose!" };
                break
            }
            _ => ()
        }

        'player_entry : loop {
            println!( "" );
            game_box.print_active_tiles();
            game_box.print_inactive_tiles();
            println!( "d1: {d1}, d2: {d2}, sum: {dice_total}" );
            println!( "Enter up to 3 numbers that add up to the dice sum" );
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
}
