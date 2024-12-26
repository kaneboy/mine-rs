use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
struct Cell {
    bomb    : bool,
    opened  : bool,
}

pub struct Field {
    pub rows        : usize,
    columns         : usize,
    cells           : Vec<Cell>,
    cursor_row      : usize,
    cursor_col      : usize,
}

impl Field {
    pub fn new(rows: usize, columns: usize, bomb_percentage: usize) -> Self {

        let mut cells: Vec<Cell> = (0..rows*columns)
            .map(|_| Cell { bomb: false, opened: false })
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

                let cell = self.get_cell(row as isize, col as isize).unwrap();

                if row == self.cursor_row && col == self.cursor_col { print!("["); }
                else { print!(" "); }

                if !cell.opened   { print!("#"); }
                else if cell.bomb { print!("*"); }
                else              { print!("{}", self.get_bomb_count_in_nbors(row as isize, col as isize)); }

                if row == self.cursor_row && col == self.cursor_col { print!("]"); }
                else { print!(" "); }
            }
            println!();
        }
    }

    fn get_bomb_count_in_nbors(&self, row: isize, col: isize) -> usize {
        let mut result = 0usize;
        for r in row-1..=row+1 {
            for c in col-1..=col+1 {
                if r == row && c == col {
                    continue;
                }
                if let Some(cell) = self.get_cell(r, c) {
                    if cell.bomb {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    pub fn move_cursor(&mut self, row_offset: isize, col_offset: isize) {
        let mut cursor_row = self.cursor_row as isize + row_offset;
        let mut cursor_col = self.cursor_col as isize + col_offset;
        cursor_row = std::cmp::max(cursor_row, 0);
        cursor_row = std::cmp::min(cursor_row, (self.rows - 1) as isize);
        cursor_col = std::cmp::max(cursor_col, 0);
        cursor_col = std::cmp::min(cursor_col, (self.columns - 1) as isize);
        self.cursor_row = cursor_row as usize;
        self.cursor_col = cursor_col as usize;
    }

    /// 返回 false 表示踩到地雷了。
    pub fn open(&mut self) -> bool {
        let cell = self.get_cell_mut(self.cursor_row as isize, self.cursor_col as isize).unwrap();
        cell.opened = true;
        !cell.bomb
    }

    pub fn all_opened(&self) -> bool {
        let unopened_count = self.cells.iter().filter(|c| !c.opened).count();
        let bomb_count = self.cells.iter().filter(|c| c.bomb).count();
        unopened_count == bomb_count
    }

    fn get_cell(&self, row: isize, column: isize) -> Option<&Cell> {
        if row >= 0 && row < self.rows as isize && column >= 0 && column < self.columns as isize {
            Some(&self.cells[row as usize * self.columns + column as usize])
        } else {
            None
        }
    }

    fn get_cell_mut(&mut self, row: isize, column: isize) -> Option<&mut Cell> {
        if row >= 0 && row < self.rows as isize && column >= 0 && column < self.columns as isize {
            Some(&mut self.cells[row as usize * self.columns + column as usize])
        } else {
            None
        }
    }
}