pub mod board;
mod game_over;
pub mod generate_shapes;
pub mod player_input;
pub mod visuals;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::Board;
use player_input::click;
use visuals::{resize_notificator, ScaleFactor};

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
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system(resize_notificator)
        .add_system(click)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ScaleFactor::default());
    commands.spawn(Board::default());
}
