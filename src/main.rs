use std::io;

const BEGINNER :i32 = 0;
const INTERMEDIATE :i32 = 1;
const ADVANCED :i32 = 2;
const MAXSIDE :i32 = 25;
const MAXMINES :i32 = 99;
const MOVESIZE :i32 = 526;

pub struct MineGame {
    side :i32,
    mines :i32,
} 

impl MineGame {
    pub fn is_valid(&self, row: i32, col :i32) -> bool {
        if (row >= 0 && row < self.side) && (col >= 0 && col < self.side) {
            return true;
        } else {
            return false;
        }
    }
}


fn main() {
    let mine = MineGame {
        side: 3,
    }
}
