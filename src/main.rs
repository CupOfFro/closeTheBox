use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    'main_game_loop: loop {
        let d1 = rand::thread_rng().gen_range( 1..=6 );
        let d2 = rand::thread_rng().gen_range( 1..=6 );
        let sum = d1 + d2;

        'player_input: loop {
            println!( "d1:{d1}, d2:{d2}, sum:{sum}" );

            let mut choice1 = String::new();
            let mut choice2 = String::new();
            println!( "Enter choice 1" );
            io::stdin().read_line( &mut choice1 ).expect( "Failed to read line" );
            println!( "Enter choice 2" );
            io::stdin().read_line( &mut choice2 ).expect( "Failed to read line" );

            let choice1: u32 = match choice1.trim().parse()
            {
                Ok( num ) => num,
                Err( _ ) => {
                    println!( "Invalid choice 2! Re-enter!" );
                    continue
                }
            };

            let choice2: u32 = match choice2.trim().parse()
            {
                Ok( num ) => num,
                Err( _ ) => {
                    println!( "Invalid choice 2! Re-enter!" );
                    continue
                }
            };

            let player_sum = choice1 + choice2;
            match player_sum.cmp( &sum )
            {
                Ordering::Equal => break,
                _ => {
                    println!( "Player sum does not match dice!" );
                    continue
                }
            }
        }
    }
}
