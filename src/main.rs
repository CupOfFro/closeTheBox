use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
I need these two traits to use the enum as an array element
*/
#[derive(Copy, Clone)]
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
}

fn main() {
    // Make the box
    let mut game_box: GameBox = GameBox::init();
    game_box.print_active_tiles();
    game_box.print_inactive_tiles();

    let d1 = rand::thread_rng().gen_range( 1..=6 );
    let d2 = rand::thread_rng().gen_range( 1..=6 );
    let dsum = d1 + d2;
    println!( "d1: {d1} d2: {d2} sum:{dsum}" );

    // let mut selections = String::new();
    // io::stdin().read_line( &,ut selections ).expect( "Failed to read line" );
}
