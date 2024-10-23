use crate::engine::piece::Piece;

struct Position {
    pub squares: [Piece; 64],
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    ep_file: i8,
    white_to_move: bool,
    ply_count: u16,
}

impl Position {
    fn new() -> Position {
        Position {
            squares: [Piece::None; 64],
            white_castle_kingside: false,
            white_castle_queenside: false,
            black_castle_kingside: false,
            black_castle_queenside: false,
            ep_file: -1,
            white_to_move: true,
            ply_count: 0
        }
    }

}

/*fn position_from_fen(fen: &String)  {
    let pos = Position::new();
    let mut elements: Vec<String> = vec![];
    for symbol in fen {
        if symbol ==

    }

}