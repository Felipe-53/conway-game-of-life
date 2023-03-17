struct GameOfLife {
    vector: Vec<Cell>,
}
type Cell = (i32, i32);

impl GameOfLife {
    fn new(pattern: Vec<Cell>) -> GameOfLife {
        GameOfLife { vector: pattern }
    }

    fn get_max_coordinates(&self) -> (i32, i32) {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in &self.vector {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        return (max_x, max_y);
    }

    fn get_num_of_alive_nieghbours_cells(&self, (x, y): Cell) -> i32 {
        let mut count = 0;

        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if let Some(_) = &self.vector.iter().position(|&c| c == (i, j)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn chekck_if_alive(&self, cell: Cell) -> bool {
        if let Some(_) = &self.vector.iter().position(|&c| c == cell) {
            return true;
        }

        false
    }

    fn iterate(&self) -> GameOfLife {
        let (max_x, max_y) = self.get_max_coordinates();

        let mut marked_to_be_murdered: Vec<Cell> = Vec::new();
        let mut marked_to_be_brought_to_life: Vec<Cell> = Vec::new();

        for i in 0..=max_x {
            for j in 0..=max_y {
                let is_alive = self.chekck_if_alive((i, j));
                let num_of_alive_cells = self.get_num_of_alive_nieghbours_cells((i, j));

                if is_alive {
                    if !(num_of_alive_cells == 2 || num_of_alive_cells == 3) {
                        marked_to_be_murdered.push((i, j));
                    }
                }

                if !is_alive {
                    if num_of_alive_cells == 3 {
                        marked_to_be_brought_to_life.push((i, j));
                    }
                }
            }
        }

        let mut new_pattern: Vec<Cell>;

        new_pattern = self.vector.clone();

        for cell in &marked_to_be_brought_to_life {
            new_pattern.push(cell.clone());
        }

        for cell in marked_to_be_murdered {
            if let Some(index) = self.vector.iter().position(|&c| c == cell) {
                new_pattern.remove(index);
            }
        }

        GameOfLife::new(new_pattern)
    }
}

fn main() {
    let life = GameOfLife::new(vec![(0, 0), (1, 0), (2, 0)]);

    println!("{:?}", life.iterate().vector);

    println!("{:?}", life.get_max_coordinates());
}
