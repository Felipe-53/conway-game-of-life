struct GameOfLife {
    vector: Vec<Cell>,
}
type Cell = (i32, i32);

impl GameOfLife {
    fn new(pattern: Vec<Cell>) -> GameOfLife {
        GameOfLife { vector: pattern }
    }

    fn get_min_and_max_coordinates(&self) -> [(i32, i32); 2] {
        let mut max_x = 0;
        let mut max_y = 0;

        let mut min_x = 0;
        let mut min_y = 0;

        for (x, y) in &self.vector {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
        }

        if min_x == max_x {
            min_x = -1;
            max_x = 1;
        }

        if min_y == max_y {
            min_y = -1;
            max_y = 1;
        }

        return [(min_x - 1, min_y - 1), (max_x + 1, max_y + 1)];
    }

    fn get_num_of_alive_nieghbours_cells(&self, (x, y): Cell) -> i32 {
        let mut count = 0;

        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if i == x && j == y {
                    continue;
                };
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
        let [(min_x, min_y), (max_x, max_y)] = self.get_min_and_max_coordinates();

        let mut marked_to_stay_alive: Vec<Cell> = Vec::new();
        let mut marked_to_be_brought_to_life: Vec<Cell> = Vec::new();

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                let is_alive = self.chekck_if_alive((i, j));
                let num_of_alive_cells = self.get_num_of_alive_nieghbours_cells((i, j));

                if is_alive {
                    if num_of_alive_cells == 2 || num_of_alive_cells == 3 {
                        marked_to_stay_alive.push((i, j));
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

        new_pattern = marked_to_stay_alive.clone();

        for cell in &marked_to_be_brought_to_life {
            new_pattern.push(cell.clone());
        }

        GameOfLife::new(new_pattern)
    }

    fn print(&self) {
        let [(min_x, min_y), (max_x, max_y)] = self.get_min_and_max_coordinates();

        for j in (min_y..=max_y).rev() {
            for i in min_x..=max_x {
                let cell = (i, j);
                if self.chekck_if_alive(cell) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut generation = GameOfLife::new(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);

    for i in 1..=10 {
        println!("Current generation: {}", i);
        generation.print();
        generation = generation.iterate()
    }
}
