use crate::game_box::{GameBox, Tile};
use rand::Rng;

pub fn versus() {
    // Make GameBox
    println!("In versus P1 is trying to move all tiles to inactive");
    println!("and P2 is trying to get all tiles to active");
    let mut game_box: GameBox = GameBox::init();

    loop {
        println!("");
        println!("========== P1 ==========");
        let d1: usize;
        let d2: usize;
        let dice_total: usize;

        d1 = rand::thread_rng().gen_range(1..=6);
        d2 = rand::thread_rng().gen_range(1..=6);
        dice_total = d1 + d2;

        match game_box.check_if_possible(&dice_total, Tile::Inactive) {
            false => {
                game_box.print_active_tiles();
                game_box.print_inactive_tiles();
                println!("(d1: {d1}) + (d2: {d2}) = (sum: {dice_total})");
                println!("No available moves for player 1!");
            }
            _ => game_box.player_entry(d1, d2, Tile::Active),
        }

        if game_box.get_number_of_tiles(Tile::Active) == 0 {
            println!("Player 1 wins!");
            break;
        }

        println!("");
        println!("========== P2 ==========");
        let d1: usize;
        let d2: usize;
        let dice_total: usize;

        d1 = rand::thread_rng().gen_range(1..=6);
        d2 = rand::thread_rng().gen_range(1..=6);
        dice_total = d1 + d2;

        match game_box.check_if_possible(&dice_total, Tile::Active) {
            false => {
                game_box.print_active_tiles();
                game_box.print_inactive_tiles();
                println!("(d1: {d1}) + (d2: {d2}) = (sum: {dice_total})");
                println!("No available moves for player 2!");
            }
            _ => game_box.player_entry(d1, d2, Tile::Inactive),
        }

        if game_box.get_number_of_tiles(Tile::Inactive) == 0 {
            println!("Player 2 wins!");
            break;
        }
    }
}
