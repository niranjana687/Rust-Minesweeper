#[derive(Debug, Copy, Clone)]

pub struct MineGame {
    row: i32,
    col: i32,
    mines: i32,

}

pub struct Game  {

}

pub enum Cell {
    Empty,
    Mine,
    MineNeighbor {count: u8},
}
#[allow(unused_varaiables)]

const BEGINNER :MineGame = MineGame {
    width: 9,
    height: 9,
    mines: 10,
};
// const INTERMEDIATE :MineGame = MineGame {
//     width: 16,
//     height: 16,
//     mines: 40,
// };

// const ADVANCED :MineGame = MineGame {
//     width: 24,
//     height: 24,
//     mines: 99,
// };

impl MineGame {
    
    // fn game_level(level: i8) -> MineGame {
    //     match level {
    //         0 => BEGINNER,
    //         1 => INTERMEDIATE,
    //         2 => ADVANCED,
    //         _ => panic!("Invalid input. 
    //         Try 0-> Beginner
    //             1-> Intermediate
    //             2-> Advanced"),
    //     }

    // }

    pub fn print(&self) {
        print!("┌");
        for _ in 0..self.col{
            print!("─");
        }
        println!("┐");
        for y in 0..self.row {
            print!("│");
            for x in 0..self.col {
                
    
            println!("│");
        }
        print!("└");
        for _ in 0..self.col {
            print!("─");
        }
        println!("┘");
    }
    


}

pub struct PlayBoard {
    
}

fn main() {
 loop {
    println!(" \tM I N E S W E E P E R\n
    \n
    COMMANDS: 
    \n
    row col -> reveal row, col\n
    m row col-> mark row, col\n
    q-> quit\n
    n-> new game\n
    b-> Beginner\n
    i-> Intermediate\n
    a-> Advanced\n

   
");
 }
}