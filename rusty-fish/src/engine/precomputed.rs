pub struct Precomputed {
    pub directional_offset: [i8; 8],
    pub squares_to_edge: [[i8;8]; 64],
}

impl Precomputed {
    fn is_at_edge(&self, index: i8, direction: i8) -> bool {
        match direction {
            8   => index + 8 > 63,
            -8  => index - 8 < 0,
            -1  => index % 8 == 0,
            1   => index % 8 == 7,
            7   => self.is_at_edge(index, -1)   || self.is_at_edge(index, 8),
            9   => self.is_at_edge(index, 1)    || self.is_at_edge(index, 8),
            -7  => self.is_at_edge(index, 1)    || self.is_at_edge(index, -8),
            -9  => self.is_at_edge(index, -1)   || self.is_at_edge(index, -8),
            _   => false,
        }

    }

    fn calculate_squares_to_edge(&mut self) {
        for i in 0..64 {
            for j in 0..8 {
                let direction = self.directional_offset[j];
                let mut count = 0;
                let mut moved = i;
                while(!self.is_at_edge(moved, direction)) {
                    count += 1;
                    moved += direction;
                }
                self.squares_to_edge[i as usize][j as usize] = count;
            }
        }
    }
    pub fn new() -> Precomputed {
        Precomputed {
            directional_offset: [8, -8, -1, 1, 7, 9, -7, -9],
            squares_to_edge: [[0; 8]; 64],
        }
    }

    pub fn init(&mut self) {
        self.calculate_squares_to_edge();
    }
}