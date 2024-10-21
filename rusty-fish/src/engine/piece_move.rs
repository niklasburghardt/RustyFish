enum Flag {
    None,
    EP,
    CastleKing,
    CastleQueen,
    Promotion,
    DoublePawnPush,
    KingMove,
}

enum Promotion {
    None,
    Knight,
    Bishop,
    Rook,
    Queen,
}

struct PieceMove {
    start: i8,
    end: i8,
    flag: Flag,
    promotion: Promotion,
}