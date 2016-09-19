// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    direction: Direction,
    x: isize,
    y: isize,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { direction: d, x: x, y: y }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { direction: new_direction, x: self.x, y: self.y }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot { direction: new_direction, x: self.x, y: self.y }
    }

    pub fn advance(self) -> Self {
        let new_coordinate = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };
        Robot { direction: self.direction, x: new_coordinate.0, y: new_coordinate.1 }
    }

    #[allow(unused_variables)]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                'A' => self = self.advance(),
                _ => self = self
            }
        }
        self
    }

    pub fn position(&self) -> (isize, isize) {
       (self.x, self.y) 
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
