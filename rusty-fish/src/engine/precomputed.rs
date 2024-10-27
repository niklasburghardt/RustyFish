use crate::utils::board_representation::calculate_distance;

pub struct Precomputed {
    pub directional_offset: [i8; 8],
    pub squares_to_edge: [[i8;8]; 64],
    pub knight_offsets: [i8; 8],
    pub knight_moves: [[i8;8]; 64],
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

    fn calculate_knight_moves(&mut self) {
        for i in 0..64 {
            for j in 0..8 {
                let offset = self.knight_offsets[j];
                let target = i+offset;
                if target < 0 || target > 63 {
                    continue;
                }
                if calculate_distance(i as u8, target as u8) != 3 {
                    continue;
                }
                self.knight_moves[i as usize][j] = target;
            }
        }
    }
    pub fn new() -> Precomputed {
        Precomputed {
            directional_offset: [8, -8, -1, 1, 7, 9, -7, -9],
            squares_to_edge: [[0; 8]; 64],
            knight_offsets: [6, 15, 17, 10, -6, -15, -17, -10],
            knight_moves: [[-1; 8]; 64],
        }
    }

    pub fn init(&mut self) {
        self.calculate_squares_to_edge();
        self.calculate_knight_moves();
    }
}