use graphics::types::Color;
use graphics::{Context, Graphics};

use piston::input::UpdateArgs;

pub struct GameboardSettings {
    pub grid_color: Color,
}

impl GameboardSettings {
    pub fn new() -> GameboardSettings {
        GameboardSettings {
            grid_color: [0.0, 0.0, 0.0, 0.8],
        }
    }
}

#[derive(PartialEq, Clone)]
enum CellState {
    Alive,
    Dead,
}

pub struct Gameboard {
    pub settings: GameboardSettings,
    updates: i64,
    number_of_rows: i64,
    number_of_cols: i64,
    cells: Vec<CellState>,
}

impl Gameboard {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardSettings) -> Gameboard {
        let number_of_rows = 80;
        let number_of_cols = 120;
        let cells = (0..number_of_cols * number_of_rows)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            })
            .collect();

        Gameboard {
            settings,
            updates: 0,
            number_of_rows,
            number_of_cols,
            cells,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};
        let [view_width, view_height] = c.get_view_size();

        let cell_size;
        if (view_width / self.number_of_cols as f64) < (view_height / self.number_of_rows as f64) {
            cell_size = (view_width / self.number_of_cols as f64).floor();
        } else {
            cell_size = (view_height / self.number_of_rows as f64).floor();
        }

        let cell_edge = Line::new(self.settings.grid_color, 1.0);
        for i in 0..self.number_of_cols {
            let x = i as f64 * cell_size;
            let vline = [x, 0.0, x, self.number_of_rows as f64 * cell_size];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);
        }

        for i in 0..self.number_of_rows {
            let y = i as f64 * cell_size;
            let hline = [0.0, y, self.number_of_cols as f64 * cell_size, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        for (index, cell) in self.cells.iter().enumerate() {
            let (row, col) = getPositionFromIndex(self.number_of_cols as f64, index as f64);
            //println!("Index: {:?} Row: {:?} / {:?}", index, row, col);

            if *cell == CellState::Alive {
                Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(
                    [
                        col as f64 * cell_size,
                        row as f64 * cell_size,
                        cell_size,
                        cell_size,
                    ],
                    &c.draw_state,
                    c.transform,
                    g,
                );
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.updates += 1;
        let mut old_cells = self.cells.clone();

        for (index, cell) in self.cells.iter_mut().enumerate() {

            let mut number_of_living_cells_near_by = 0;
            let current_position = getPositionFromIndex(self.number_of_cols as f64, index as f64);

            for index in 0..8 {
                let row = current_position.0 + (index as f64 / 3.0).floor() as i64;
                let col = current_position.1 + index % 3;

                if current_position == (row, col) {
                    continue;
                }

                let neihgbour_cell = old_cells.get(getPositionFromRowCol(self.number_of_cols, row, col) as usize);

                if let Some(CellState::Alive) = neihgbour_cell {
                    number_of_living_cells_near_by += 1;
                }

                if number_of_living_cells_near_by > 4 {
                    break;
                }
            }

            if *cell == CellState::Alive {
                if number_of_living_cells_near_by > 3 || number_of_living_cells_near_by < 2 {
                *cell = CellState::Dead;
                } else if number_of_living_cells_near_by == 2  || number_of_living_cells_near_by == 3 {
                    *cell = CellState::Alive;
                }
            } else if number_of_living_cells_near_by == 3 {
                *cell = CellState::Alive;
            }

            if let Some(mut zell) = old_cells.get(index) {
                zell = &cell.clone();
            }

            // if *cell == CellState::Alive {
            //     *cell = CellState::Dead;
            // } else {
            //     *cell = CellState::Alive;
            // }
        }
    }
}

 fn getPositionFromIndex(number_of_cols: f64, index: f64) -> (i64, i64) {
        let row = (index / number_of_cols).floor();
        let col = index - row as f64 * number_of_cols;

        return (row as i64, col as i64);
    }

    fn getPositionFromRowCol(number_of_cols: i64, row: i64, col: i64) -> i64 {
        return row * number_of_cols + col;
    }
