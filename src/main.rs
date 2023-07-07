pub mod board;
pub mod game_over;
pub mod generate_shapes;
pub mod player_input;
pub mod scale;
pub mod visuals;
pub mod start_screen;
mod main_menu;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::Board;
use player_input::click;
use scale::{resize, ScaleFactor};
use start_screen::{create_start_screen, button_system};
use visuals::{create_board, create_grid_cover};

// The size of each cell
const CELL_SIZE: f32 = 60.0;
// The distance between the cell and the grid
const CELL_PADDING: f32 = 6.0;
// The thickness of the grid lines
const GRID_LINE_THICKNESS: f32 = 6.0;
// Text size
const TEXT_SIZE: f32 = 60.0;
// Cross color
const CROSS_COLOR: Color = Color::rgb(0.3, 0.3, 0.85);
// Nought color
const NOUGHT_COLOR: Color = Color::rgb(0.85, 0.3, 0.3);
// The thickness of the lines in the cross and nought
const CROSS_AND_NOUGHT_LINE_THICKNESS: f32 = 10.0;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .add_startup_system(create_grid_cover)
        .add_startup_system(create_start_screen)
        .add_system(resize)
        .add_system(button_system)
        .add_system(click)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Board::default());

    let scale_factor = ScaleFactor::default();

    commands.spawn(scale_factor);
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GaneOver,
}