use std::env;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]

pub struct MineGame {
    row: i32,
    column: i32,
    mines: i32,
}

impl MineGame {
    
    pub fn game_print(&self) {
        for _ in 0..self.row {
            for _ in 1..self.column{
                print!(".");
            }println!(".");
        }
    }

    pub fn mine_generate(&self) -> HashMap<i32,i32 > {
        //rows
        let mut rng_r = rand::thread_rng();
        let x: f64 = rng_r.gen();
        let mut r :Vec<i32> = (1..10).collect();
        r.shuffle(&mut rng_r);

        //columns
        let mut rng_c = rand::thread_rng();
        let y: f64 = rng_c.gen();
        let mut c :Vec<i32> = (1..10).collect();
        c.shuffle(&mut rng_c);

        //mine grids
        let mut mine_grid: HashMap<_,_> =
            r.into_iter().zip(c.into_iter()).collect();
        mine_grid

    }

    
}

pub struct PlayBoard {}

fn main() {
    let game = MineGame {
        row: 9,
        column: 9,
        mines: 10,
    };
    println!("\t\t M I N E S W E E P E R ");
    game.game_print();
    println!();
    println!("\tThe number of mines to be found: {}", game.mines);
    println!("\tThe rules of game is as follows\n\t-m  row column:  marks mine\n\t-row column: marks free space");
    println!("\t-If you need to reveal the mines before you play, there is a quick option 🙊. Type \"cheat\"");  
    



}
