use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
struct Cell {
    bomb    : bool,
    opened  : bool,
}

pub struct Field {
    rows        : usize,
    columns     : usize,
    cells       : Vec<Cell>,
    cursor_row  : usize,
    cursor_col  : usize,
}

impl Field {
    pub fn new(rows: usize, columns: usize, bomb_percentage: usize) -> Self {

        let mut cells: Vec<Cell> = (0..rows*columns)
            .map(|_| Cell { bomb: false, opened: true })
            .collect();

        let bomb_count = rows * columns * bomb_percentage / 100;
        let bomb_count = std::cmp::max(bomb_count, 0);
        let bomb_count = std::cmp::min(bomb_count, rows * columns);

        let mut rng = rand::thread_rng();
        let cell_count = cells.len();

        for _ in 0..bomb_count {
            loop {
                let cell = &mut cells[rng.gen_range(0..cell_count)];
                if !cell.bomb {
                    cell.bomb = true;
                    break;
                }
            }
        }

        Self {
            rows,
            columns,
            cells,
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.columns {

                let cell = self.get_cell(row, col);

                if row == self.cursor_row && col == self.cursor_col { print!("["); }
                else { print!(" "); }

                if !cell.opened   { print!("#"); }
                else if cell.bomb { print!("*"); }
                else              { print!(" "); }

                if row == self.cursor_row && col == self.cursor_col { print!("]"); }
                else { print!(" "); }
            }
            println!();
        }
    }

    pub fn move_cursor(&mut self, row_offset: isize, col_offset: isize) {
        
    }

    fn get_cell(&self, row: usize, column: usize) -> &Cell {
        &self.cells[row * self.columns + column]
    }

    fn get_cell_mut(&mut self, row: usize, column: usize) -> &mut Cell {
        &mut self.cells[row * self.columns + column]
    }
}