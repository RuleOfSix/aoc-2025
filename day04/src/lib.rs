use std::fs;

pub struct RollMap {
    grid: Vec<Vec<MapCell>>,
}

pub struct Coords {
    x: usize,
    y: usize,
}

impl RollMap {
    fn new(grid_string: String) -> Self {
        RollMap {
            grid: grid_string
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars())
                .map(|s| s.map(|c| MapCell::new(c)).collect())
                .collect(),
        }
    }

    pub fn remove_rolls(&mut self, cs: Vec<Coords>) {
        for c in cs {
            self.remove_roll(c);
        }
    }

    fn remove_roll(&mut self, c: Coords) {
        if let Some(b) = self.grid.get_mut(c.x).and_then(|v| v.get_mut(c.y)) {
            *b = MapCell::Empty;
        }
    }

    pub fn get_reachable(&self) -> Vec<Coords> {
        let mut reachables = Vec::new();
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let cell = Coords { x, y };
                match self.is_reachable(&cell) {
                    Some(true) => reachables.push(cell),
                    _ => (),
                }
            }
        }
        reachables
    }

    fn is_reachable(&self, c: &Coords) -> Option<bool> {
        Some(
            *self.grid.get(c.x)?.get(c.y)? == MapCell::Roll
                && self.count_neighbors_of_type(c, MapCell::Roll) < 4,
        )
    }

    fn count_neighbors_of_type(&self, c: &Coords, t: MapCell) -> u8 {
        let mut count: u8 = 0;
        for dx in -1_isize..=1 {
            for dy in -1_isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                count += match self
                    .grid
                    .get((c.x as isize + dx) as usize)
                    .and_then(|v| v.get((c.y as isize + dy) as usize))
                {
                    Some(mc) => {
                        if *mc == t {
                            1
                        } else {
                            0
                        }
                    }
                    None => 0,
                }
            }
        }
        count
    }
}

#[derive(PartialEq, Eq, Debug)]
enum MapCell {
    Empty,
    Roll,
}

impl MapCell {
    fn new(c: char) -> Self {
        match c {
            '@' => Self::Roll,
            _ => Self::Empty,
        }
    }
}

pub fn read_input() -> RollMap {
    RollMap::new(fs::read_to_string("./input.txt").expect("Should be able to read input file"))
}
