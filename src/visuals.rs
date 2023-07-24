use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    board::{CrossOrNought, GridState},
    scale::{Scale, TextScale, ScaleFactor},
    shapes::{generate_cross_path, generate_nought_path},
    Board, BOTTOM_TEXT_SIZE, CELL_PADDING, CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS, CROSS_COLOR,
    GRID_LINE_THICKNESS, NOUGHT_COLOR, TOP_TEXT_SIZE, loading::FontAssets,
};

const GRID_COVER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.20);

pub fn spawn_board(mut commands: Commands, q_scale_factor: Query<&ScaleFactor>, font_assets: Res<FontAssets>) {
    let scale_fac = q_scale_factor.single().0;

    // Grid lines
    for x in 0..2 {
        for y in -4..4 {
            let pos = 0.5 * (CELL_SIZE + GRID_LINE_THICKNESS)
                + CELL_PADDING
                + y as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS);

            let scale = 9.0 * (CELL_SIZE + 2.0 * CELL_PADDING) + 8.0 * (GRID_LINE_THICKNESS);

            let translation = Vec3 {
                x: if x == 1 { pos * scale_fac } else { 0.0 },
                y: if x == 0 { pos * scale_fac } else { 0.0 },
                z: if (y + 5) % 3 == 0 { 1.0 } else { 0.0 },
            };

            let scale = Vec3 {
                x: if x == 1 { GRID_LINE_THICKNESS * scale_fac } else { scale * scale_fac },
                y: if x == 0 { GRID_LINE_THICKNESS * scale_fac } else { scale * scale_fac },
                z: 1.0,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if (y + 5) % 3 == 0 {
                            Color::rgb(0.5, 0.3, 0.3)
                        } else {
                            Color::rgb(0.6, 0.6, 0.6)
                        },
                        ..default()
                    },
                    transform: Transform {
                        translation,
                        scale,
                        ..Default::default()
                    },
                    ..default()
                },
                Scale,
            ));
        }
    }

    // Top text
    let top_text_style = TextStyle {
        font: font_assets.poppins_semi_bold.clone(),
        font_size: TOP_TEXT_SIZE * scale_fac,
        color: Color::rgb(0.1, 0.1, 0.1),
    };

    // Bottom text
    let bottom_text_style = TextStyle {
        font: font_assets.poppins_medium.clone(),
        font_size: BOTTOM_TEXT_SIZE * scale_fac,
        color: Color::rgb(0.1, 0.1, 0.1),
    };

    // Top text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Welcome to Ultimate Tic Tac Toe", top_text_style),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    5.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                    0.0,
                ),
                ..Default::default()
            },
            ..default()
        },
        TextScale,
    ));

    // Bottom text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("X's turn", bottom_text_style),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    (-5.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) 
                        + GRID_LINE_THICKNESS) * scale_fac,
                    0.0,
                ),
                ..Default::default()
            },
            ..default()
        },
        TextScale,
        BottomText,
    ));

    // Creats the cells?
    /*
    for x in -4..5 {
        for y in -4..5 {
            commands.spawn((SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: x as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                        y: y as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                        z: 0.0,
                    },
                    scale: Vec3::splat(CELL_SIZE * scale_fac),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            }, GridLines));
        }
    }
    */
}

pub fn update_bottom_text(
    mut q_bottom_text: Query<&mut Text, With<BottomText>>,
    q_board: Query<&Board>,
    mut last_player_turn: Local<CrossOrNought>,
) {
    let player_turn = q_board.single().player_turn();
    if *player_turn != *last_player_turn {
        match player_turn {
            CrossOrNought::Cross => {
                q_bottom_text.single_mut().sections[0].value = "X's turn".to_owned()
            }
            CrossOrNought::Nought => {
                q_bottom_text.single_mut().sections[0].value = "O's turn".to_owned()
            }
        }
        *last_player_turn = player_turn.to_owned();
    }
}

pub fn spawn_grid_cover(mut commands: Commands, q_scale_factor: Query<&ScaleFactor>, ) {
    let scale_fac = q_scale_factor.single().0;

    // Invisible grid covers
    for x in -1i32..2i32 {
        for y in -1i32..2i32 {
            let translation = Vec3::new(
                (x * 3) as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                (y * 3) as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                10.0,
            );

            let scale = Vec3::new(
                (3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) - GRID_LINE_THICKNESS) * scale_fac,
                (3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) - GRID_LINE_THICKNESS) * scale_fac,
                1.0,
            );

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::NONE,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation,
                        scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Scale,
                GridCover((x + 1) as u8 + (y + 1) as u8 * 3),
            ));
        }
    }
}

pub fn update_grid_cover(board: &Board, mut q_grid_covers: Query<(&mut Sprite, &GridCover)>) {
    for (mut sprite, grid_cover) in &mut q_grid_covers {
        match board.state_for_grid(grid_cover.0 as usize) {
            GridState::Active => sprite.color = Color::NONE,
            GridState::Inactive => sprite.color = GRID_COVER_COLOR,
            GridState::Tie => sprite.color = GRID_COVER_COLOR.with_a(0.4),
            GridState::WonByCross => sprite.color = CROSS_COLOR.with_a(0.2),
            GridState::WonByNought => sprite.color = NOUGHT_COLOR.with_a(0.25),
        }
    }
}

pub fn reset_grid_cover(mut q_grid_covers: Query<&mut Sprite, With<GridCover>>) {
    for mut sprite in &mut q_grid_covers {
        sprite.color = Color::NONE;
    }
}

pub fn spawn_symbol(
    commands: &mut Commands,
    x: f32,
    y: f32,
    scale_fac: f32,
    player_turn: CrossOrNought,
) {
    let translation = Vec3 {
        x: x * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        y: y * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        z: 0.0,
    };

    commands.spawn((
        ShapeBundle {
            path: match player_turn {
                CrossOrNought::Cross => {
                    generate_cross_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
                }
                CrossOrNought::Nought => {
                    generate_nought_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
                }
            },
            transform: Transform {
                translation: translation * scale_fac,
                scale: Vec3::splat(scale_fac),
                ..Default::default()
            },
            ..Default::default()
        },
        Fill::color(match player_turn {
            CrossOrNought::Cross => CROSS_COLOR,
            CrossOrNought::Nought => NOUGHT_COLOR,
        }),
        Scale,
        Symbol,
    ));
}

pub fn spawn_large_symbol(
    commands: &mut Commands,
    x: f32,
    y: f32,
    scale_fac: f32,
    player_turn: CrossOrNought,
) {
    let large_x = ((x + 4.0) / 3.0).floor() - 1.0;
    let large_y = ((y + 4.0) / 3.0).floor() - 1.0;

    let translation = Vec3 {
        x: large_x * 3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        y: large_y * 3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        z: 15.0,
    };

    commands.spawn((
        ShapeBundle {
            path: match player_turn {
                CrossOrNought::Cross => {
                    generate_cross_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
                }
                CrossOrNought::Nought => {
                    generate_nought_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
                }
            },
            transform: Transform {
                translation: translation * scale_fac,
                scale: Vec3::splat(3.0 * scale_fac),
                ..Default::default()
            },
            ..Default::default()
        },
        Fill::color(match player_turn {
            CrossOrNought::Cross => CROSS_COLOR,
            CrossOrNought::Nought => NOUGHT_COLOR,
        }),
        Scale,
        Symbol,
    ));
}

pub fn despawn_symbols(commands: &mut Commands, q_symbols: Query<Entity, With<Symbol>>) {
    for symbol in &q_symbols {
        commands.entity(symbol).despawn();
    }
}

#[derive(Component)]
pub struct GridCover(u8);

#[derive(Component)]
pub struct Symbol;

#[derive(Component)]
pub struct BottomText;
