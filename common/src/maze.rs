use rand::Rng;
use rand::seq::SliceRandom;

pub const NORTH: u8 = 1;
pub const EAST: u8  = 2;
pub const SOUTH: u8 = 4;
pub const WEST: u8  = 8;

fn opposite(dir: u8) -> u8 {
    match dir {
        NORTH => SOUTH,
        EAST  => WEST,
        SOUTH => NORTH,
        WEST  => EAST,
        _     => 0,
    }
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<u8>,
}

impl Maze {
    pub fn generate(width: usize, height: usize, braid_factor: f32, rng: &mut impl Rng) -> Self {
        let mut cells = vec![0u8; width * height];
        let mut visited = vec![false; width * height];
        let mut stack = Vec::new();

        let (sx, sy) = (rng.gen_range(0..width), rng.gen_range(0..height));
        visited[sy * width + sx] = true;
        stack.push((sx, sy));

        while let Some(&(x, y)) = stack.last() {
            let neighbors = Self::unvisited_neighbors(x, y, width, height, &visited);
            if neighbors.is_empty() {
                stack.pop();
            } else {
                let &(nx, ny, dir) = neighbors.choose(rng).unwrap();
                cells[y * width + x] |= dir;
                cells[ny * width + nx] |= opposite(dir);
                visited[ny * width + nx] = true;
                stack.push((nx, ny));
            }
        }

        Self::braid(&mut cells, width, height, braid_factor, rng);
        Self { width, height, cells }
    }

    fn braid(cells: &mut Vec<u8>, width: usize, height: usize, factor: f32, rng: &mut impl Rng) {
        for y in 0..height {
            for x in 0..width {
                let open = [NORTH, EAST, SOUTH, WEST]
                    .iter()
                    .filter(|&&d| cells[y * width + x] & d != 0)
                    .count();

                if open == 1 && rng.gen_range(0.0..1.0) < factor {
                    let neighbors: Vec<(usize, usize, u8)> = vec![
                        (x, y.wrapping_sub(1), NORTH),
                        (x, y + 1, SOUTH),
                        (x + 1, y, EAST),
                        (x.wrapping_sub(1), y, WEST),
                    ]
                    .into_iter()
                    .filter(|&(nx, ny, dir)| nx < width && ny < height && cells[y * width + x] & dir == 0)
                    .collect();

                    if let Some(&(nx, ny, dir)) = neighbors.choose(rng) {
                        cells[y * width + x]   |= dir;
                        cells[ny * width + nx] |= opposite(dir);
                    }
                }
            }
        }
    }

    fn unvisited_neighbors(x: usize, y: usize, width: usize, height: usize, visited: &[bool]) -> Vec<(usize, usize, u8)> {
        let mut neighbors = Vec::new();
        if y > 0 && !visited[(y-1) * width + x] { neighbors.push((x, y-1, NORTH)); }
        if y + 1 < height && !visited[(y+1) * width + x] { neighbors.push((x, y+1, SOUTH)); }
        if x + 1 < width && !visited[y * width + (x+1)] { neighbors.push((x+1, y, EAST)); }
        if x > 0 && !visited[y * width + (x-1)] { neighbors.push((x-1, y, WEST)); }
        neighbors
    }

    pub fn has_passage(&self, x: usize, y: usize, dir: u8) -> bool {
        self.cells[y * self.width + x] & dir != 0
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}