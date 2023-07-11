pub mod board;
pub mod game_over;
pub mod generate_shapes;
pub mod menu;
pub mod player_input;
pub mod scale;
pub mod visuals;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::Board;
use menu::MenuPlugin;
use player_input::main_mouse_system;
use scale::{resize, ScaleFactor};
use visuals::{
    despawn_symbols, reset_grid_cover, spawn_board, spawn_grid_cover, GridCover, Symbol,
};

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
        // Bevy Plugins
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // My Plugins
        .add_plugin(MenuPlugin)
        // Startup Systems
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_grid_cover)
        // Systems
        .add_system(resize)
        .add_system(reset_board.in_schedule(OnEnter(AppState::Game)))
        .add_system(main_mouse_system.in_set(OnUpdate(AppState::Game)))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Board::default());
    commands.spawn(ScaleFactor::default());
}

fn reset_board(
    mut commands: Commands,
    q_symbols: Query<Entity, With<Symbol>>,
    mut q_board: Query<&mut Board>,
    q_grid_covers: Query<&mut Sprite, With<GridCover>>,
) {
    q_board.single_mut().reset();

    despawn_symbols(&mut commands, q_symbols);
    reset_grid_cover(q_grid_covers);
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
