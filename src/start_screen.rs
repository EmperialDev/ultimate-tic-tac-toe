use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes};

use crate::{
    game_over::TryAgainButton,
    scale::{BetterScale, TextScale}, board::Board,
};

// The who won text
pub const WHO_WON_TEXT_SIZE: f32 = 60.0;
// The retry text
pub const TRY_AGAIN_TEXT_SIZE: f32 = 40.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn create_start_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Poppins-Medium.ttf"),
        font_size: WHO_WON_TEXT_SIZE,
        color: Color::rgb(0.95, 0.95, 0.95),
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.6).into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(16.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        text: Text::from_section("Welcome to", text_style.clone()),
                        ..Default::default()
                    },
                    TextScale::new(Vec3::splat(0.0), WHO_WON_TEXT_SIZE)
                ));
                    parent.spawn((TextBundle {
                        text: Text::from_section("Ultiamte Tic Tac Toe", text_style),
                        ..Default::default()
                    },
                    TextScale::new(Vec3::splat(0.0), WHO_WON_TEXT_SIZE)
                ));
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Button",
                                TextStyle {
                                    font: asset_server.load("fonts/Poppins-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut q_board: Query<&mut Board>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                q_board.single_mut().reset();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

//---------------------------

pub fn start_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shape = shapes::RoundedPolygon {
        points: [
            Vec2::new(WHO_WON_TEXT_SIZE * 4.0, WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(-WHO_WON_TEXT_SIZE * 4.0, WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(-WHO_WON_TEXT_SIZE * 4.0, -WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(WHO_WON_TEXT_SIZE * 4.0, -WHO_WON_TEXT_SIZE * 0.8),
        ]
        .map(|f| f)
        .into_iter()
        .collect(),
        radius: 10.0,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Fill::color(Color::rgba(0.95, 0.95, 0.95, 0.7)),
        BetterScale::from_location(Vec3::new(0.0, WHO_WON_TEXT_SIZE, 15.0)),
    ));

    let shape = shapes::RoundedPolygon {
        points: [
            Vec2::new(TRY_AGAIN_TEXT_SIZE * 2.5, TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(-TRY_AGAIN_TEXT_SIZE * 2.5, TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(-TRY_AGAIN_TEXT_SIZE * 2.5, -TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(TRY_AGAIN_TEXT_SIZE * 2.5, -TRY_AGAIN_TEXT_SIZE * 0.8),
        ]
        .map(|f| f)
        .into_iter()
        .collect(),
        radius: 10.0,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Fill::color(Color::rgba(0.95, 0.95, 0.95, 0.7)),
        BetterScale::from_location(Vec3::new(0.0, -TRY_AGAIN_TEXT_SIZE, 15.0)),
        TryAgainButton::default(),
    ));

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/Poppins-Medium.ttf"),
        font_size: WHO_WON_TEXT_SIZE,
        color: Color::rgb(0.1, 0.1, 0.1),
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press play to start", text_style),
            ..Default::default()
        },
        TextScale::new(Vec3::new(0.0, WHO_WON_TEXT_SIZE, 20.0), WHO_WON_TEXT_SIZE),
    ));

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/Poppins-Light.ttf"),
        font_size: TRY_AGAIN_TEXT_SIZE,
        color: Color::rgb(0.25, 0.25, 0.25),
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Try again", text_style),
            ..Default::default()
        },
        TextScale::new(
            Vec3::new(0.0, -TRY_AGAIN_TEXT_SIZE, 20.0),
            TRY_AGAIN_TEXT_SIZE,
        ),
    ));
}
