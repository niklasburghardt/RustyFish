pub enum Flag {
    None,
    EP,
    CastleKing,
    CastleQueen,
    Promotion,
    DoublePawnPush,
    KingMove,
}

pub enum Promotion {
    None,
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub struct PieceMove {
    pub start: usize,
    pub end: usize,
    pub flag: Flag,
    pub promotion: Promotion,
}