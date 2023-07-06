use bevy::{prelude::*, window::WindowResized};
use bevy_prototype_lyon::prelude::*;

use crate::{
    board::{Cell, GridState},
    game_over::game_over_text,
    generate_shapes::{generate_cross_path, generate_nought_path},
    Board, CELL_PADDING, CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS, CROSS_COLOR,
    GRID_LINE_THICKNESS, NOUGHT_COLOR, TEXT_SIZE,
};

const GRID_COVER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.15);

pub fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_lines_query: Query<Entity, With<Scale>>,
    mut scale_factor_query: Query<&mut ScaleFactor>,
    q_board: Query<&Board>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        clear_board(&mut commands, &grid_lines_query);

        let scale_num_x =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 2.5 * TEXT_SIZE;
        let scale_num_y =
            9.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) + 4.0 * TEXT_SIZE;

        let scale_x = e.height / scale_num_x;
        let scale_y = e.width / scale_num_y;

        let scale_fac = if scale_x < scale_y { scale_x } else { scale_y };

        let mut scale_factor = scale_factor_query.single_mut();
        scale_factor.scale = scale_fac;
        scale_factor.screen_width = e.width;
        scale_factor.screen_height = e.height;

        let board = q_board.single();

        create_board(&mut commands, &asset_server, scale_factor.scale);
        create_grid_cover(&mut commands, board, scale_factor.scale);
        place_symbols(&mut commands, board, scale_factor.scale);
        if !board.game_active() {
            game_over_text(
                &mut commands,
                &asset_server,
                board.board_won_by(),
                scale_fac,
            );
        }
    }
}

fn clear_board(commands: &mut Commands, grid_lines_query: &Query<Entity, With<Scale>>) {
    let mut counter = 0;
    for entity in grid_lines_query {
        commands.entity(entity).despawn();
        counter += 1;
    }

    //println!("Despawned {counter}");
}

fn create_board(commands: &mut Commands, asset_server: &Res<AssetServer>, scale_fac: f32) {
    // Grid lines
    for x in 0..2 {
        for y in -4..4 {
            let pos = 0.5 * (CELL_SIZE + GRID_LINE_THICKNESS)
                + CELL_PADDING
                + y as f32 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS);

            let scale = 9.0 * (CELL_SIZE + 2.0 * CELL_PADDING) + 8.0 * (GRID_LINE_THICKNESS);

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: if x == 1 { pos * scale_fac } else { 0.0 },
                            y: if x == 0 { pos * scale_fac } else { 0.0 },
                            z: if (y + 5) % 3 == 0 { 1.0 } else { 0.0 },
                        },
                        scale: Vec3 {
                            x: if x == 1 {
                                GRID_LINE_THICKNESS * scale_fac
                            } else {
                                scale * scale_fac
                            },
                            y: if x == 0 {
                                GRID_LINE_THICKNESS * scale_fac
                            } else {
                                scale * scale_fac
                            },
                            z: 1.0,
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: if (y + 5) % 3 == 0 {
                            Color::rgb(0.5, 0.3, 0.3)
                        } else {
                            Color::rgb(0.6, 0.6, 0.6)
                        },
                        ..default()
                    },
                    ..default()
                },
                Scale,
            ));
        }
    }

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/BAHNSCHRIFT.ttf"),
        font_size: TEXT_SIZE * scale_fac,
        color: Color::rgb(0.1, 0.1, 0.1),
    };

    // Top text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Welcome to Ultimate Tic Tac Toe", text_style),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 5.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Scale,
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

fn create_grid_cover(commands: &mut Commands, board: &Board, scale_fac: f32) {
    // Invisible grid covers
    for x in -1..2 {
        for y in -1..2 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            ((x * 3) as f32
                                * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS)
                                - GRID_LINE_THICKNESS / 4.0 * x as f32)
                                * scale_fac,
                            ((y * 3) as f32
                                * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS)
                                - GRID_LINE_THICKNESS / 4.0 * y as f32)
                                * scale_fac,
                            10.0,
                        ),
                        scale: Vec3::new(
                            (3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS)
                                - GRID_LINE_THICKNESS / 2.0 * (x as f32).abs())
                                * scale_fac,
                            (3.0 * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS)
                                - GRID_LINE_THICKNESS / 2.0 * (y as f32).abs())
                                * scale_fac,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: match board.state_for_grid((x + 1 + (y + 1) * 3) as usize) {
                            GridState::Active => Color::rgba(0.0, 0.0, 0.0, 0.0),
                            GridState::Inactive => GRID_COVER_COLOR,
                            GridState::WonByCross => CROSS_COLOR.with_a(0.15),
                            GridState::WonByNought => NOUGHT_COLOR.with_a(0.2),
                        },
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

fn place_symbols(commands: &mut Commands, board: &Board, scale_fac: f32) {
    for (x, grid) in board.grid().iter().enumerate() {
        for (y, cell) in grid.iter().enumerate() {
            if cell != &Cell::Empty {
                let x_grid = ((x % 3) * 3 + y % 3) as f32;
                let y_grid = (x as f32 / 3.0).floor() * 3.0 + (y as f32 / 3.0).floor();

                place_symbol_single(commands, x_grid - 4.0, y_grid - 4.0, scale_fac, cell);
            }
        }
    }
}

pub fn place_symbol_single(commands: &mut Commands, x: f32, y: f32, scale_fac: f32, cell: &Cell) {
    if cell == &Cell::Empty {
        error!("Tried to spawn an empty symbol!");
        return;
    }

    commands.spawn((
        ShapeBundle {
            path: if cell == &Cell::Cross {
                generate_cross_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
            } else {
                generate_nought_path(CELL_SIZE, CROSS_AND_NOUGHT_LINE_THICKNESS)
            },
            transform: Transform {
                translation: Vec3 {
                    x: x * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                    y: y * (CELL_SIZE + 2.0 * CELL_PADDING + GRID_LINE_THICKNESS) * scale_fac,
                    z: 0.0,
                },
                scale: Vec3::splat(scale_fac),
                ..default()
            },
            ..Default::default()
        },
        Fill::color(if cell == &Cell::Cross {
            CROSS_COLOR
        } else {
            NOUGHT_COLOR
        }),
        Scale,
    ));
}

#[derive(Component)]
pub struct Scale;

#[derive(Component)]
pub struct GridCover(u8);

#[derive(Component, Default)]
pub struct ScaleFactor {
    pub scale: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}
