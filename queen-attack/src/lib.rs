static MAX_X: i8 = 8;
static MAX_Y: i8 = 8;

pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, &'static str> {
        if x >= 0 && x < MAX_X && y >= 0 && y < MAX_Y {
            return Ok(ChessPosition { x: x, y: y });
        }
        Err("Invalid Position")
    }
}

pub struct Queen {
    chess_position: ChessPosition,
}

impl Queen {
    pub fn new(chess_position: ChessPosition) -> Self {
        Queen { chess_position: chess_position }
    }

    pub fn can_attack(&self, other_queen: &Queen) -> bool {
        if other_queen.chess_position.x == self.chess_position.x ||
           other_queen.chess_position.y == self.chess_position.y {
            return true;
        } else {
            let x_diff = other_queen.chess_position.x - self.chess_position.x;
            if self.chess_position.y + x_diff == other_queen.chess_position.y ||
               self.chess_position.y - x_diff == other_queen.chess_position.y {
                return true;
            }
        }
        return false;
    }
}
