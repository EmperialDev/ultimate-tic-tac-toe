use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    board::{Cell, GridState},
    generate_shapes::{generate_cross_path, generate_nought_path},
    scale::{Scale, TextScale},
    Board, CELL_PADDING, CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS, CROSS_COLOR,
    GRID_LINE_THICKNESS, NOUGHT_COLOR, TEXT_SIZE,
};

const GRID_COVER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.15);

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Grid lines
    for x in 0..2 {
        for y in -4..4 {
            let pos = 0.5 * (CELL_SIZE + GRID_LINE_THICKNESS)
                + CELL_PADDING
                + y as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS);

            let scale = 9.0 * (CELL_SIZE + 2.0 * CELL_PADDING) + 8.0 * (GRID_LINE_THICKNESS);

            let translation = Vec3 {
                x: if x == 1 { pos } else { 0.0 },
                y: if x == 0 { pos } else { 0.0 },
                z: if (y + 5) % 3 == 0 { 1.0 } else { 0.0 },
            };

            let scale = Vec3 {
                x: if x == 1 { GRID_LINE_THICKNESS } else { scale },
                y: if x == 0 { GRID_LINE_THICKNESS } else { scale },
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

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/Poppins-SemiBold.ttf"),
        font_size: TEXT_SIZE,
        color: Color::rgb(0.1, 0.1, 0.1),
    };

    let translation = Vec3 {
        x: 0.0,
        y: 5.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        z: 0.0,
    };
    // Top text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Welcome to Ultimate Tic Tac Toe", text_style),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..default()
        },
        TextScale,
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

pub fn spawn_grid_cover(mut commands: Commands) {
    // Invisible grid covers
    for x in -1i32..2i32 {
        for y in -1i32..2i32 {
            let translation = Vec3::new(
                (x * 3) as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
                (y * 3) as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
                10.0,
            );

            let scale = Vec3::new(
                3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) - GRID_LINE_THICKNESS,
                3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) - GRID_LINE_THICKNESS,
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
            GridState::Active => sprite.color = Color::rgba(0.0, 0.0, 0.0, 0.0),
            GridState::Inactive => sprite.color = GRID_COVER_COLOR,
            GridState::WonByCross => sprite.color = CROSS_COLOR.with_a(0.15),
            GridState::WonByNought => sprite.color = NOUGHT_COLOR.with_a(0.2),
        }
    }
}

pub fn reset_grid_cover(mut q_grid_covers: Query<&mut Sprite, With<GridCover>>) {
    for mut sprite in &mut q_grid_covers {
        sprite.color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    }
}

pub fn spawn_symbol(commands: &mut Commands, x: f32, y: f32, scale_fac: f32, cell: &Cell) {
    if cell == &Cell::Empty {
        error!("Tried to spawn an empty symbol!");
        return;
    }

    let translation = Vec3 {
        x: x * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        y: y * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS),
        z: 0.0,
    };

    commands.spawn((
        ShapeBundle {
            path: if cell == &Cell::Cross {
                generate_cross_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
            } else {
                generate_nought_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
            },
            transform: Transform {
                translation: translation * scale_fac,
                scale: Vec3::splat(scale_fac),
                ..Default::default()
            },
            ..Default::default()
        },
        Fill::color(if cell == &Cell::Cross {
            CROSS_COLOR
        } else {
            NOUGHT_COLOR
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
