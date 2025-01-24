use std::io;
use std::cmp::Ordering;
use rand::Rng;

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
                    print!( "{tile_index}" );
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
}

fn main() {
    // Make the box
    let mut game_box: GameBox = GameBox::init();

    loop {
        if game_box.get_number_of_active_tiles() == 0 {
            println!( "YOU WIN!" );
            break;
        }

        game_box.print_active_tiles();
        game_box.print_inactive_tiles();

        let d1 = rand::thread_rng().gen_range( 1..=6 );
        let d2 = rand::thread_rng().gen_range( 1..=6 );
        let dice_total = d1 + d2;
        println!( "d1: {d1} d2: {d2} sum: {dice_total}" );

        loop {
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
            let mut player_total = 0;
            // now we can iterate through it
            for num in selections {
                number_of_entries += 1;
                if number_of_entries > 3 {
                    println!( "Too many values entered! Must be 4 or less!" );
                    break;
                }
                let num: u8 = match num.trim().parse()
                {
                    Ok( num ) => {
                        println!( "{num}" );
                        num
                    }
                    Err(_) => {
                        println!( "{num} was not a valid number!" );
                        break;
                    }
                };
                player_total += num;
            }
            
            if player_total == dice_total {
                break;
            }
        }
    }
}
