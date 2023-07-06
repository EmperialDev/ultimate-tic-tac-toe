use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes};

use crate::{board::CrossOrNought, visuals::Scale, CROSS_COLOR, NOUGHT_COLOR};

// The who won text
pub const WHO_WON_TEXT_SIZE: f32 = 60.0;
// The retry text
pub const TRY_AGAIN_TEXT_SIZE: f32 = 40.0;

pub fn game_over_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    winner: Option<CrossOrNought>,
    scale_fac: f32,
) {
    let shape = shapes::RoundedPolygon {
        points: [
            Vec2::new(WHO_WON_TEXT_SIZE * 4.0, WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(-WHO_WON_TEXT_SIZE * 4.0, WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(-WHO_WON_TEXT_SIZE * 4.0, -WHO_WON_TEXT_SIZE * 0.8),
            Vec2::new(WHO_WON_TEXT_SIZE * 4.0, -WHO_WON_TEXT_SIZE * 0.8),
        ]
        .map(|f| f * scale_fac)
        .into_iter()
        .collect(),
        radius: 10.0 * scale_fac,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0.0, WHO_WON_TEXT_SIZE * scale_fac, 15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Fill::color(Color::rgba(0.95, 0.95, 0.95, 0.6)),
        Scale,
    ));

    let shape = shapes::RoundedPolygon {
        points: [
            Vec2::new(TRY_AGAIN_TEXT_SIZE * 2.5, TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(-TRY_AGAIN_TEXT_SIZE * 2.5, TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(-TRY_AGAIN_TEXT_SIZE * 2.5, -TRY_AGAIN_TEXT_SIZE * 0.8),
            Vec2::new(TRY_AGAIN_TEXT_SIZE * 2.5, -TRY_AGAIN_TEXT_SIZE * 0.8),
        ]
        .map(|f| f * scale_fac)
        .into_iter()
        .collect(),
        radius: 10.0 * scale_fac,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0.0, -TRY_AGAIN_TEXT_SIZE * scale_fac, 15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Fill::color(Color::rgba(0.95, 0.95, 0.95, 0.6)),
        Scale,
        TryAgainButton::default(),
    ));

    let (color, text) = if let Some(winner) = winner {
        match winner {
            CrossOrNought::Cross => (CROSS_COLOR, "X won the game"),
            CrossOrNought::Nought => (NOUGHT_COLOR, "O won the game"),
        }
    } else {
        (Color::rgb(0.1, 0.1, 0.1), "Press play to start")
    };

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/BAHNSCHRIFT.ttf"),
        font_size: WHO_WON_TEXT_SIZE * scale_fac,
        color,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(text, text_style),
            transform: Transform {
                translation: Vec3::new(0.0, WHO_WON_TEXT_SIZE * scale_fac, 20.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Scale,
    ));

    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/BAHNSCHRIFT.ttf"),
        font_size: TRY_AGAIN_TEXT_SIZE * scale_fac,
        color: Color::rgb(0.25, 0.25, 0.25),
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Try again", text_style),
            transform: Transform {
                translation: Vec3::new(0.0, -TRY_AGAIN_TEXT_SIZE * scale_fac, 20.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Scale,
    ));
}

#[derive(Component, Default)]
pub struct TryAgainButton{
    pub focus: bool
}