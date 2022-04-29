// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTION_CYCLE: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

/// find direction cycle index and turn. return the direction after instruction turn.
fn turn_cycle(n: usize, opt: &str) -> Direction {
    let nd: usize;
    if opt == "left" {
        nd = n + 3;
    } else {
        nd = n + 1;
    }
    let i = (nd % 4) as usize;
    DIRECTION_CYCLE[i]
}

#[derive(Debug)]
pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        // unimplemented!("Create a robot at (x, y) ({}, {}) facing {:?}", x, y, d,)
        Self {
            position: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            direction: turn_cycle(self.direction as usize, "right"),
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            direction: turn_cycle(self.direction as usize, "left"),
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut position = self.position;
        match self.direction {
            Direction::North => position.1 += 1,
            Direction::East => position.0 += 1,
            Direction::South => position.1 -= 1,
            Direction::West => position.0 -= 1,
        }
        Self { position, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // unimplemented!(
        //     "Follow the given sequence of instructions: {}",
        //     instructions
        // )
        instructions.chars().fold(self, |robot, ch| match ch {
            'A' => robot.advance(),
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            _ => {
                panic!("unkonw instructions `{}`!", ch)
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
