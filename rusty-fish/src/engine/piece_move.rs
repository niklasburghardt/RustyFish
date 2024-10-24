use crate::engine::precomputed::Precomputed;

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
    pub start: u8,
    pub end: u8,
    pub flag: Flag,
    pub promotion: Promotion,
}

impl PieceMove {
    pub fn simple(start: u8, end: u8) -> PieceMove {
        PieceMove {
            start,
            end,
            flag: Flag::None,
            promotion: Promotion::None
        }
    }
}