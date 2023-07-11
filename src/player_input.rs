use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    board::{Board, Cell},
    scale::ScaleFactor,
    visuals::{spawn_symbol, update_grid_cover, GridCover},
    AppState, CELL_PADDING, CELL_SIZE, GRID_LINE_THICKNESS,
};

pub fn main_mouse_system(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_scale_factor: Query<&ScaleFactor>,
    mut q_board: Query<&mut Board>,
    mut commands: Commands,
    q_grid_covers: Query<(&mut Sprite, &GridCover)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right) {
        if let Ok(window) = q_windows.get_single() {
            if let Some(position) = window.physical_cursor_position() {
                let scale_factor = q_scale_factor.single();
                let scale_fac = scale_factor.0;

                let position = position
                    - Vec2 {
                        x: window.physical_width() as f32 / 2.0,
                        y: window.physical_height() as f32 / 2.0,
                    };
                let x = (position.x
                    / ((CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac)
                    + 0.5)
                    .floor();
                let y = (position.y
                    / ((CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac)
                    + 0.5)
                    .floor();

                if !(-4.0..=4.0).contains(&x) || !(-4.0..=4.0).contains(&y) {
                    return;
                }

                let cell = if buttons.just_pressed(MouseButton::Left) {
                    Cell::Cross
                } else {
                    Cell::Nought
                };

                let mut board = q_board.single_mut();
                if board.place_symbol(x, y, &cell, &mut app_state_next_state) {
                    spawn_symbol(&mut commands, x, y, scale_fac, &cell);
                    update_grid_cover(&board, q_grid_covers);
                }
            } else {
                println!("Cursor out of window")
            }
        } else {
            println!("Coun't get window")
        };
    };
}
