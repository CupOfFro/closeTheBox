/*
So this doesn't really work without the rest of the game loop
in main, so I am just declaring the module in main.
*/


// No need for this as gamebox is declared in main.rs
// pub mod game_box {

use std::io;

// I need these two traits to use the enum as an array element
#[derive(Copy, Clone, PartialEq)]
// Below is my Tile enum. A tile can be active or inactive
pub enum Tile {
    Active,
    Inactive
}

// My struct for the gameboard. It holds 9 Tiles
pub struct GameBox {
    pub tiles: Vec< Tile >
}

impl GameBox {
    // Create a new gameboard and return self
    pub fn init() -> Self {
        let game_board_size: usize;
        loop {
            println!( "Enter game board size 9(default) - 12" );
            let mut game_board_size_input = String::new();
            io::stdin().read_line( &mut game_board_size_input ).expect( "Failed to read line" );
            game_board_size = match game_board_size_input.trim().parse() {
                Ok( game_board_size ) => {
                    if game_board_size >= 9 && game_board_size <= 12 {
                        game_board_size
                    }
                    else {
                        println!( "Game board size must be between 1 and 12!" );
                        continue;
                    }
                }
                Err( _ ) => {
                    println!( "Did not get a number!" );
                    continue;
                }
            };
            // range check
            
            break;
        }
        Self {
            tiles: vec![ Tile::Active; game_board_size ]
        }
    }

    pub fn print_active_tiles( &self ) {
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

    pub fn print_inactive_tiles( &self ) {
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

    pub fn get_number_of_tiles( &self, tile: Tile ) -> i32 {
        let mut count = 0;
        for i in &self.tiles {
            if *i == tile {
                count += 1;
            }
        }
        count
    }

    // This function will take the dice roll, and see if any 1, 2, 3, or 4
    // combinations of tiles can add to the dice sum
    // Place the opposite tile type the player wants in as an argument
    pub fn check_if_possible( &self, dice_roll: &usize, check_against_tile: Tile ) -> bool {
        // println!( "Checking for {dice_roll}" );
        for i in 0..self.tiles.len() {
            if self.tiles[ i ] == check_against_tile {
                continue;
            }
            if i + 1 == *dice_roll {
                // println!( "{} matches!", i+1 );
                return true
            }
            for j in i+1..self.tiles.len() {
                if self.tiles[ j ] == check_against_tile {
                    continue;
                }
                if i + 1 + j + 1 == *dice_roll {
                    // println!( "{} {} matches!", i+1, j+1 );
                    return true
                }
                for k in j+1..self.tiles.len() {
                    if self.tiles[ k ] == check_against_tile {
                        continue;
                    }
                    if i + 1 + j + 1 + k + 1 == *dice_roll {
                        // println!( "{} {} {} matches!", i+1, j+1, k+1 );
                        return true
                    }
                }
            }
        }
        false
    }

    /*
    If all values above 6 are inactive, the player can decide to roll
    a single die
    */
    pub fn check_if_single_die_possible( &self, target_tile: Tile ) -> bool {
        for t in &self.tiles[ 6.. ] {
            if target_tile == Tile::Active {
                if *t == Tile::Active {
                    return false
                }
            }
            else {
                if *t == Tile::Inactive {
                    return false
                }
            }
        }

        true
    }

    pub fn player_entry( &mut self, d1: usize, d2: usize, target_tile: Tile )
    {
        const ALLOWED_NUMBER_OF_ENTRIES: usize = 3;
        let dice_total = d1 + d2;

        'player_entry : loop {
            println!( "" );
            self.print_active_tiles();
            self.print_inactive_tiles();
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
            
            let mut player_entries: [ usize; ALLOWED_NUMBER_OF_ENTRIES ] = [ 0; ALLOWED_NUMBER_OF_ENTRIES ];
            let mut input_index = 0;
            // now we can iterate through it
            for num in selections {
                number_of_entries += 1;
                if number_of_entries > ALLOWED_NUMBER_OF_ENTRIES {
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
                for j in i+1..ALLOWED_NUMBER_OF_ENTRIES
                {
                    if *val == player_entries[ j ] {
                        if *val == 0 || player_entries[ j ] == 0 {
                            continue;
                        }
                        println!( "duplicate detected! {val} and {}", player_entries[j] );
                        continue 'player_entry;
                    }
                }
            }

            // Check if entries are all Active Tiles
            for entry in player_entries {
                if entry == 0 {
                    continue;
                }
                if entry > self.tiles.len() {
                    println!( "{entry} is not a valid tile number!" );
                    continue;
                }
                if target_tile == Tile::Active{
                    if self.tiles[ entry - 1 ] != Tile::Active {
                        println!( "Tile {entry} is not active! Enter and active tile!" );
                        continue 'player_entry;
                    }
                }
                else {
                    if self.tiles[ entry - 1 ] != Tile::Inactive {
                        println!( "Tile {entry} is not inactive! Enter and inactive tile!" );
                        continue 'player_entry;
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
                    if target_tile == Tile::Active {
                        self.tiles[ val - 1 ] = Tile::Inactive;
                    }
                    else {
                        self.tiles[ val - 1 ] = Tile::Active;
                    }
                }
                break;
            }
            else {
                println!( "Entry did not add up to dice sum!" );
            }
        }
    }
}
