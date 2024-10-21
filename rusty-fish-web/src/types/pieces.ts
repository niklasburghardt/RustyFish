export enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

export enum Color {
    White,
    Black,
} 

export type ChessPiece = {
    piece: Piece;
    color: Color;
}; 
