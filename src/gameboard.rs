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

#[derive(PartialEq)]
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
            let row = (index as f64 / self.number_of_cols as f64).floor();
            let col = index as f64 - row * self.number_of_cols as f64;
            //println!("Index: {:?} Row: {:?} / {:?}", index, row, col);

            if *cell == CellState::Alive {
                Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
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
        for (index, cell) in self.cells.iter_mut().enumerate() {
            if *cell == CellState::Alive {
                *cell = CellState::Dead;
            } else {
                *cell = CellState::Alive;
            }
        }
    }
}
