#[derive(Debug)]

pub struct MineGame {
    row: i32,
    column: i32,
    mines: i32,
}

impl MineGame {
    fn game_level(level: i8) -> Self {
        match level {
            0 => Self::beginner(),
            1 => Self::intermediate(),
            2 => Self::advanced(),
            _ => panic!(
                "Invalid input. 
            Try 0-> Beginner
                1-> Intermediate
                2-> Advanced"
            ),
        }
    }

    const fn beginner() -> Self {
        Self {
            row: 9,
            column: 9,
            mines: 10,
        }
    }

    const fn intermediate() -> Self {
        Self {
            row: 16,
            column: 16,
            mines: 40,
        }
    }

    const fn advanced() -> Self {
        Self {
            row: 24,
            column: 24,
            mines: 99,
        }
    }
}

pub struct PlayBoard {}

fn main() {}
