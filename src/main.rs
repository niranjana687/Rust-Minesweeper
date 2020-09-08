use std::io;
use std::time::Duration;

const BEGINNER :i32 = 0;
const INTERMEDIATE :i32 = 1;
const ADVANCED :i32 = 2;
const MAXSIDE :i32 = 25;
const MAXMINES :i32 = 99;
const MOVESIZE :i32 = 526;
//stroes player info
pub struct Player {
    name: String,
    age: u8,
    time: Duration,
    //wins in one game = 1 point
    score: i32,
}

pub struct MineGame {
    side :i32,
    mines :i32,
    player: Player,
} 

pub struct RealBoard{
    side: i32,
    mines: i32,
}



impl MineGame {
    pub fn is_valid(&self, row: i32, col :i32) -> bool {
        if (row >= 0 && row < self.side) && (col >= 0 && col < self.side) {
            return true;
        } else {
            return false;
        }
    }

    pub fn choose_difficulty(level: String) -> MineGame {

    }
    pub fn cheat_minesweeper() ->
}


fn main() {
    let mine = MineGame {
        side: 3,
    }
}
