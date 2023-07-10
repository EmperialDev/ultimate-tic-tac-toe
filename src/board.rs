use bevy::prelude::*;
use std::fmt::Debug;

use crate::AppState;

#[derive(Component, Default)]
pub struct Board {
    grid: [[Cell; 9]; 9],
    grid_state: [GridState; 9],
    player_turn: CrossOrNought,
    last_grid: Option<usize>,
}

impl Board {
    /// Takes in the pos of the symbol
    ///
    /// `x` and `y`: range: -4..5
    pub fn place_symbol(&mut self, x: f32, y: f32, cell: &Cell, app_state: &mut NextState<AppState>) -> bool {

        let index_in_grid = (x + 4.0) as usize % 3 + ((y + 4.0) as usize % 3) * 3;
        let grid_index = (((x + 4.0) / 3.0).floor() + ((y + 4.0) / 3.0).floor() * 3.0) as usize;

        if self.grid[grid_index][index_in_grid] != Cell::Empty {
            return false;
        }
        if self.state_for_grid(grid_index) != &GridState::Active {
            return false;
        }

        self.grid[grid_index][index_in_grid] = *cell;

        if let Some(last_grid) = self.last_grid {
            if let Some(won_by) = Self::check_if_won(&self.grid[last_grid]) {
                self.grid_state[last_grid] = match won_by {
                    CrossOrNought::Cross => GridState::WonByCross,
                    CrossOrNought::Nought => GridState::WonByNought,
                };

                if Self::check_if_won(&self.grid_state.map(|f| f.into())).is_some() {
                    println!("You won!");
                    app_state.set(AppState::GaneOver);
                }
            }
        }
        self.last_grid = Some(index_in_grid);

        if self.grid_state[index_in_grid] == GridState::WonByCross
            || self.grid_state[index_in_grid] == GridState::WonByNought
        {
            for i in 0..9 {
                if self.grid_state[i] == GridState::Inactive {
                    self.grid_state[i] = GridState::Active;
                }
            }
        } else {
            for i in 0..9 {
                if i == index_in_grid {
                    self.grid_state[i] = GridState::Active;
                } else if self.grid_state[i] == GridState::Active {
                    self.grid_state[i] = GridState::Inactive;
                }
            }
        }

        self.player_turn = match self.player_turn {
            CrossOrNought::Cross => CrossOrNought::Nought,
            CrossOrNought::Nought => CrossOrNought::Cross,
        };

        true
    }

    /// Resets the board
    pub fn reset(&mut self) {
        self.grid = [[Cell::default(); 9]; 9];
        self.grid_state = [GridState::default(); 9];
        self.player_turn = CrossOrNought::Cross;
        self.last_grid = None;
    }

    /// Returns the grid
    pub fn grid(&self) -> &[[Cell; 9]; 9] {
        &self.grid
    }

    // Returns wich player's turn it is
    pub fn player_turn(&self) -> &CrossOrNought {
        &self.player_turn
    }

    /// Returns the specified `GridState`
    pub fn state_for_grid(&self, index: usize) -> &GridState {
        &self.grid_state[index]
    }

    /// Returns if anyone won
    pub fn board_won_by(&self) -> Option<CrossOrNought> {
        Self::check_if_won(&self.grid_state.map(|f| f.into()))
    }

    fn check_if_won(grid: &[Cell; 9]) -> Option<CrossOrNought> {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        for i in 0..3 {
            if grid[i * 3 + 1] != Cell::Empty
                && grid[i * 3] == grid[i * 3 + 1]
                && grid[i * 3 + 1] == grid[i * 3 + 2]
            {
                match grid[i * 3 + 1] {
                    Cell::Cross => return Some(CrossOrNought::Cross),
                    Cell::Nought => return Some(CrossOrNought::Nought),
                    _ => unreachable!(),
                }
            }
        }

        // 0 3 6
        // 1 4 7
        // 2 5 8
        for i in 0..3 {
            if grid[i + 3] != Cell::Empty && grid[i] == grid[i + 3] && grid[i + 3] == grid[i + 6] {
                match grid[i + 3] {
                    Cell::Cross => return Some(CrossOrNought::Cross),
                    Cell::Nought => return Some(CrossOrNought::Nought),
                    _ => unreachable!(),
                }
            }
        }

        if grid[4] != Cell::Empty {
            // 0 4 8
            if grid[0] == grid[4] && grid[4] == grid[8] {
                match grid[4] {
                    Cell::Cross => return Some(CrossOrNought::Cross),
                    Cell::Nought => return Some(CrossOrNought::Nought),
                    _ => unreachable!(),
                }
            }

            // 2 4 6
            if grid[2] == grid[4] && grid[4] == grid[6] {
                match grid[4] {
                    Cell::Cross => return Some(CrossOrNought::Cross),
                    Cell::Nought => return Some(CrossOrNought::Nought),
                    _ => unreachable!(),
                }
            }
        }

        None
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cell {
    #[default]
    Empty,
    Cross,  // X
    Nought, // O
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum CrossOrNought {
    #[default]
    Cross,
    Nought,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum GridState {
    #[default]
    Active,
    Inactive,
    WonByCross,
    WonByNought,
}

impl From<GridState> for Cell {
    fn from(val: GridState) -> Self {
        match val {
            GridState::Active | GridState::Inactive => Cell::Empty,
            GridState::WonByCross => Cell::Cross,
            GridState::WonByNought => Cell::Nought,
        }
    }
}

#[allow(unused)]
fn debug_print_cell_array(grid: &[Cell; 9]) {
    println!("-----");
    print!("[");

    for (i, cell) in grid.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("]");
            print!("[")
        }
        match cell {
            Cell::Empty => print!("."),
            Cell::Cross => print!("x"),
            Cell::Nought => print!("o"),
        }
    }

    println!("]");
}
