pub fn x_from_index(index: u8) -> u8 {
    (index % 8)
}

pub fn y_from_index(index: u8) -> u8 {
    7 - (index - x_from_index(index)) / 8
}

pub fn index_from_coords(x: u8, y: u8) -> u8 {
    (7-y) * 8 + x
}