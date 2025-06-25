struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (&'a u32, &'a u32);

    type IntoIter = GridRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridRefIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

struct GridRefIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl<'a> Iterator for GridRefIter<'a> {
    type Item = (&'a u32, &'a u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((&self.grid.x_coords[self.i], &self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_once() {
        let grid = Grid {
            x_coords: vec![3, 5, 7, 9],
            y_coords: vec![10, 20, 30, 40],
        };
        for (x, y) in grid {
            println!("point = {x}, {y}");
        }
    }

    #[test]
    fn iterate_twice() {
        let grid = Grid {
            x_coords: vec![3, 5, 7, 9],
            y_coords: vec![10, 20, 30, 40],
        };
        for (x, y) in &grid {
            println!("point = {x}, {y}");
        }
        for (x, y) in &grid {
            println!("point = {x}, {y}");
        }
    }
}
