pub struct ChessPosition {
    row: i32,
    col: i32
}

impl ChessPosition {
    pub fn new(row: i32, col: i32) -> Result<Self, String> {
        if row < 0 || row >= 8 || col < 0 || col >= 8 {
            return Err(String::from("Invalid position."));
        }

        Ok(ChessPosition {
            row: row,
            col: col
        })
    }
}

pub struct Queen {
    position: ChessPosition
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position: position
        }
    }

    pub fn can_attack(self, other: &Queen) -> bool {
        self.position.row == other.position.row ||
        self.position.col == other.position.col ||
        (self.position.row - other.position.row).abs() == (self.position.col - other.position.row).abs()
    }
}