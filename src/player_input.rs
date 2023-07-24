use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    board::Board,
    scale::ScaleFactor,
    visuals::{spawn_large_symbol, spawn_symbol, update_grid_cover, GridCover},
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
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = q_windows.get_single() {
            if let Some(position) = window.cursor_position() {
                let scale_factor = q_scale_factor.single();
                let scale_fac = scale_factor.0;

                let position = position
                    - Vec2 {
                        x: window.width() as f32 / 2.0,
                        y: window.height() as f32 / 2.0,
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

                let mut board = q_board.single_mut();
                if let Some(player_turn) = board.place_symbol(x, y, &mut app_state_next_state) {
                    spawn_symbol(&mut commands, x, y, scale_fac, player_turn);
                    update_grid_cover(&board, q_grid_covers);

                    if let Some(won_by) = board.grid_won_by(x, y) {
                        spawn_large_symbol(&mut commands, x, y, scale_fac, won_by);
                    }
                }
            } else {
                warn!("Cursor out of window")
            }
        } else {
            error!("Coun't get window")
        };
    };
}
