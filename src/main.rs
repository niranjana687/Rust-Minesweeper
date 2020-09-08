#[derive(Debug, Copy, Clone)]

pub struct MineGame {
    width: i32,
    height: i32,
    mines: i32,
}
#[!allow(unused_varaiables)]

const BEGINNER :MineGame = MineGame {
    width: 9,
    height: 9,
    mines: 10,
};
const INTERMEDIATE :MineGame = MineGame {
    width: 16,
    height: 16,
    mines: 40,
};

const ADVANCED :MineGame = MineGame {
    width: 24,
    height: 24,
    mines: 99,
};

impl MineGame {
    
    fn game_level(level: i8) -> MineGame {
        match level {
            0 => BEGINNER,
            1 => INTERMEDIATE,
            2 => ADVANCED,
            _ => panic!("Invalid input. 
            Try 0-> Beginner
                1-> Intermediate
                2-> Advanced"),
        }

    }


}

pub struct PlayBoard {
    
}

fn main() {

}