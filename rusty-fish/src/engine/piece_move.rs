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
    start: i8,
    end: i8,
    flag: Flag,
    promotion: Promotion,
}