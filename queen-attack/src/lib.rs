#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // unimplemented!(
        //     "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
        //     rank,
        //     file
        // );
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // unimplemented!(
        //     "Given the chess position {:?}, construct a Queen struct.",
        //     position
        // );
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // unimplemented!(
        //     "Determine if this Queen can attack the other Queen {:?}",
        //     other
        // );
        // no same row, column, diagonal
        self.0.rank == other.0.rank
            || self.0.file == other.0.file
            || (self.0.rank - other.0.rank).abs() == (self.0.file - other.0.file).abs()
    }
}
