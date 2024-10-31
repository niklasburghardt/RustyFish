use crate::engine::precomputed::Precomputed;
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub enum Flag {
    None,
    EP,
    CastleKing,
    CastleQueen,
    Promotion,
    DoublePawnPush,
    KingMove,
}

#[derive(Clone)]
pub enum Promotion {
    None,
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub struct PieceMove {
    pub start: i8,
    pub end: i8,
    pub flag: Flag,
    pub promotion: Promotion,
}

impl PieceMove {
    pub fn none() -> PieceMove {
        PieceMove {
            start: 0,
            end: 0,
            flag: Flag::None,
            promotion: Promotion::None
        }
    }

    pub fn clone(&self) -> PieceMove {
        PieceMove {
            start: self.start,
            end: self.end,
            flag: self.flag.clone(),
            promotion: self.promotion.clone(),
        }
    }
    pub fn simple(start: i8, end: i8) -> PieceMove {
        PieceMove {
            start,
            end,
            flag: Flag::None,
            promotion: Promotion::None
        }
    }
}