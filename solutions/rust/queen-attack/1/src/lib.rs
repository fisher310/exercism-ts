#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition(rank, file))
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.0.0 == other.0.0 || self.0.1 == other.0.1 {
            true
        } else if (self.0.0 - other.0.0).abs() == (self.0.1 - other.0.1).abs() {
            true
        } else {
            false
        }
    }
}
