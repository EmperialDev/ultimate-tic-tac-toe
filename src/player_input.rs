use bevy::{input::touch::TouchPhase, prelude::*, window::PrimaryWindow};

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
    mut q_grid_covers: Query<(&mut Sprite, &GridCover)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = q_windows.get_single() {
            if let Some(position) = window.cursor_position() {
                let position = position
                    - Vec2 {
                        x: window.width() / 2.0,
                        y: window.height() / 2.0,
                    };

                process_inputs(
                    position,
                    &q_scale_factor,
                    &mut q_board,
                    &mut commands,
                    &mut q_grid_covers,
                    &mut app_state_next_state,
                );
            } else {
                warn!("Cursor out of window");
            }
        } else {
            error!("Coun't get window");
        };
    };
}

pub fn touch_input(
    mut touch_event: EventReader<TouchInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_scale_factor: Query<&ScaleFactor>,
    mut q_board: Query<&mut Board>,
    mut commands: Commands,
    mut q_grid_covers: Query<(&mut Sprite, &GridCover)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in touch_event.iter() {
        if let TouchPhase::Started = event.phase {
            if let Ok(window) = q_windows.get_single() {
                let position = Vec2 {
                    x: event.position.x - window.width() / 2.0,
                    y: (window.height() - event.position.y) - window.height() / 2.0,
                };

                process_inputs(
                    position,
                    &q_scale_factor,
                    &mut q_board,
                    &mut commands,
                    &mut q_grid_covers,
                    &mut app_state_next_state,
                );
            } else {
                error!("Coun't get window");
            }
        }
    }
}

fn process_inputs(
    position: Vec2,
    q_scale_factor: &Query<&ScaleFactor>,
    q_board: &mut Query<&mut Board>,
    commands: &mut Commands,
    q_grid_covers: &mut Query<(&mut Sprite, &GridCover)>,
    app_state_next_state: &mut NextState<AppState>,
) {
    let scale_fac = if let Ok(scale_factor) = q_scale_factor.get_single() {
        scale_factor.0
    } else {
        1.0
    };

    let x = (position.x / ((CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac)
        + 0.5)
        .floor();
    let y = (position.y / ((CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac)
        + 0.5)
        .floor();
    if !(-4.0..=4.0).contains(&x) || !(-4.0..=4.0).contains(&y) {
        return;
    }
    let mut board = q_board.single_mut();

    if let Some(player_turn) = board.place_symbol(x, y, app_state_next_state) {
        spawn_symbol(commands, x, y, scale_fac, player_turn);
        update_grid_cover(&board, q_grid_covers);

        if let Some(won_by) = board.grid_won_by(x, y) {
            spawn_large_symbol(commands, x, y, scale_fac, won_by);
        }
    }
}
