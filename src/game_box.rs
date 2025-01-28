/*
So this doesn't really work without the rest of the game loop
in main, so I am just declaring the module in main.
*/


// No need for this as gamebox is declared in main.rs
// pub mod game_box {

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
    pub fn init( game_board_size: usize ) -> Self {
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

    pub fn get_number_of_active_tiles( &self ) -> i32 {
        let mut count = 0;
        for i in &self.tiles {
            if *i == Tile::Active {
                count += 1;
            }
        }
        count
    }

    // This function will take the dice roll, and see if any 1, 2, 3, or 4
    // combinations of tiles can add to the dice sum
    pub fn check_if_possible( &self, dice_roll: &usize ) -> u8 {
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
                }
            }
        }
        0
    }

    /*
    If all values above 6 are inactive, the player can decide to roll
    a single die
    */
    pub fn check_if_single_die_possible( &self ) -> bool {
        for t in &self.tiles[ 6.. ] {
            if *t == Tile::Active {
                return false
            }
        }

        true
    }
}
