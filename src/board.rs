use bevy::prelude::*;
use std::fmt::Debug;

use crate::AppState;

#[derive(Component, Default)]
pub struct Board {
    grid: [[Cell; 9]; 9],
    grid_state: [GridState; 9],
    player_turn: CrossOrNought,
    won_by: WinState,
}

impl Board {
    /// Takes in the pos of the symbol
    ///
    /// `x` and `y`: range: -4..5
    pub fn place_symbol(
        &mut self,
        x: f32,
        y: f32,
        app_state: &mut NextState<AppState>,
    ) -> Option<CrossOrNought> {
        let index_in_grid = (x + 4.0) as usize % 3 + ((y + 4.0) as usize % 3) * 3;
        let grid_index = (((x + 4.0) / 3.0).floor() + ((y + 4.0) / 3.0).floor() * 3.0) as usize;

        if self.grid[grid_index][index_in_grid] != Cell::Empty {
            return None;
        }
        if self.state_for_grid(grid_index) != &GridState::Active {
            return None;
        }

        self.grid[grid_index][index_in_grid] = self.player_turn.clone().into();

        let check_if_board_is_won =
            match Self::check_if_won(&self.grid[grid_index].map(|f| f.into())) {
                WinState::WonByCross => {
                    self.grid_state[grid_index] = GridState::WonByCross;
                    true
                }
                WinState::WonByNought => {
                    self.grid_state[grid_index] = GridState::WonByNought;
                    true
                }
                WinState::Tie => {
                    self.grid_state[grid_index] = GridState::Tie;
                    true
                }
                WinState::NotWon => false,
            };

        if check_if_board_is_won {
            self.won_by = *Self::check_if_won(&self.grid_state.map(|f| f.into()));

            if self.won_by != WinState::NotWon {
                app_state.set(AppState::GameOver);
            }
        }

        if self.grid_state[index_in_grid] == GridState::WonByCross
            || self.grid_state[index_in_grid] == GridState::WonByNought
            || self.grid_state[index_in_grid] == GridState::Tie
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

        let player_turn = self.player_turn.to_owned();

        self.player_turn = match self.player_turn {
            CrossOrNought::Cross => CrossOrNought::Nought,
            CrossOrNought::Nought => CrossOrNought::Cross,
        };

        Some(player_turn)
    }

    /// Resets the board
    pub fn reset(&mut self) {
        self.grid = [[Cell::default(); 9]; 9];
        self.grid_state = [GridState::default(); 9];
        self.player_turn = CrossOrNought::Cross;
        self.won_by = WinState::default();
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

    pub fn grid_won_by(&self, x: f32, y: f32) -> Option<CrossOrNought> {
        let index = (((x + 4.0) / 3.0).floor() + ((y + 4.0) / 3.0).floor() * 3.0) as usize;

        match self.grid_state[index] {
            GridState::Active | GridState::Inactive | GridState::Tie => None,
            GridState::WonByCross => Some(CrossOrNought::Cross),
            GridState::WonByNought => Some(CrossOrNought::Nought),
        }
    }

    /// Returns if anyone won
    pub fn board_won_by(&self) -> &WinState {
        &self.won_by
    }

    fn check_if_won(grid: &[WinState; 9]) -> &WinState {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        for i in 0..3 {
            if grid[i * 3 + 1] != WinState::NotWon
                && grid[i * 3] == grid[i * 3 + 1]
                && grid[i * 3 + 1] == grid[i * 3 + 2]
            {
                return &grid[i * 3 + 1];
            }
        }

        // 0 3 6
        // 1 4 7
        // 2 5 8
        for i in 0..3 {
            if grid[i + 3] != WinState::NotWon
                && grid[i] == grid[i + 3]
                && grid[i + 3] == grid[i + 6]
            {
                return &grid[i + 3];
            }
        }

        if grid[4] != WinState::NotWon {
            // 0 4 8
            if grid[0] == grid[4] && grid[4] == grid[8] {
                return &grid[4];
            }

            // 2 4 6
            if grid[2] == grid[4] && grid[4] == grid[6] {
                return &grid[4];
            }
        }

        if !grid.contains(&WinState::NotWon) {
            return &WinState::Tie;
        }

        &WinState::NotWon
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cell {
    #[default]
    Empty,
    Cross,  // X
    Nought, // O
}

impl From<Cell> for WinState {
    fn from(val: Cell) -> Self {
        match val {
            Cell::Empty => Self::NotWon,
            Cell::Cross => Self::WonByCross,
            Cell::Nought => Self::WonByNought,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum CrossOrNought {
    #[default]
    Cross,
    Nought,
}

impl From<CrossOrNought> for Cell {
    fn from(val: CrossOrNought) -> Self {
        match val {
            CrossOrNought::Cross => Self::Cross,
            CrossOrNought::Nought => Self::Nought,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum WinState {
    #[default]
    NotWon,
    WonByCross,
    WonByNought,
    Tie,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum GridState {
    #[default]
    Active,
    Inactive,
    WonByCross,
    WonByNought,
    Tie,
}

impl From<GridState> for Cell {
    fn from(val: GridState) -> Self {
        match val {
            GridState::Active | GridState::Inactive | GridState::Tie => Self::Empty,
            GridState::WonByCross => Self::Cross,
            GridState::WonByNought => Self::Nought,
        }
    }
}

impl From<GridState> for WinState {
    fn from(val: GridState) -> Self {
        match val {
            GridState::Active | GridState::Inactive => Self::NotWon,
            GridState::WonByCross => Self::WonByCross,
            GridState::WonByNought => Self::WonByNought,
            GridState::Tie => Self::Tie,
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
