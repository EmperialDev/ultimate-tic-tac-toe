use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;


use crate::{
    board::{Board, Cell},
    game_over::{TryAgainButton, TRY_AGAIN_TEXT_SIZE},
    scale::ScaleFactor,
    visuals::{place_symbol, update_grid_cover, GridCover},
    CELL_PADDING, CELL_SIZE, GRID_LINE_THICKNESS,
};

pub fn click(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    commands: Commands,
    q_board: Query<&mut Board>,
    q_scale_factor: Query<&ScaleFactor>,
    q_grid_covers: Query<(&mut Sprite, &GridCover)>,
    q_try_again_button: Query<(&mut Fill, &mut TryAgainButton)>,
) {
    if q_board.single().game_active() {
        place_symbol_on_click(
            buttons,
            q_windows,
            q_scale_factor,
            q_board,
            commands,
            q_grid_covers,
        )
    } else {
        //game_over_menu_buttons(buttons, q_windows, q_scale_factor, q_try_again_button, q_board)
    }
}

fn game_over_menu_buttons(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_scale_factor: Query<&ScaleFactor>,
    mut q_try_again_button: Query<(&mut Fill, &mut TryAgainButton)>,
    mut q_board: Query<&mut Board>,
) {
    if let Ok(window) = q_windows.get_single() {
        if let Some(position) = window.physical_cursor_position() {
            let scale_factor = q_scale_factor.single();
            let scale_fac = scale_factor.0;

            let position = position
                - Vec2::new(
                    window.physical_width() as f32 / 2.0,
                    window.physical_height() as f32 / 2.0,
                );

            let x_bounding = TRY_AGAIN_TEXT_SIZE * 2.5 * scale_fac;
            let y_bounding = TRY_AGAIN_TEXT_SIZE * 0.8 * scale_fac;

            if (-x_bounding..x_bounding).contains(&position.x)
                && (-y_bounding - TRY_AGAIN_TEXT_SIZE * scale_fac
                    ..y_bounding - TRY_AGAIN_TEXT_SIZE * scale_fac)
                    .contains(&position.y)
            {
                
                if !(q_try_again_button.single().1.focus) {
                    let (mut fill, mut try_again_button) = q_try_again_button.single_mut();

                    fill.color = Color::rgba(0.925, 0.925, 0.925, 0.7);
                    try_again_button.focus = true;
                }
                

                if buttons.just_pressed(MouseButton::Left) {
                    q_board.single_mut().reset();
                }
            } else if q_try_again_button.single().1.focus {
                let (mut fill, mut try_again_button) = q_try_again_button.single_mut();

                fill.color = Color::rgba(0.95, 0.95, 0.95, 0.7);
                try_again_button.focus = false;
            }
        }
    }
}

fn place_symbol_on_click(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_scale_factor: Query<&ScaleFactor>,
    mut q_board: Query<&mut Board>,
    mut commands: Commands,
    q_grid_covers: Query<(&mut Sprite, &GridCover)>,
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

                if q_board.single().game_active() {
                    let mut board = q_board.single_mut();
                    if board.place_symbol(x, y, &cell) {
                        place_symbol(&mut commands, x, y, scale_fac, &cell);
                        update_grid_cover(&board, q_grid_covers);
                    }
                }
            } else {
                println!("Cursor out of window")
            }
        } else {
            println!("Coun't get window")
        };
    };
}
