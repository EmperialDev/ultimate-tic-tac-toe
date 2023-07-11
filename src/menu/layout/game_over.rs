use bevy::prelude::*;

use crate::board::Board;
use crate::menu::components::*;
use crate::menu::styles::*;
use crate::scale::ScaleFactor;
use crate::scale::TextScale;
use crate::scale::UiScale;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_board: Query<&Board>,
    q_scale_factor: Query<&ScaleFactor>,
) {
    build_game_over_menu(
        &mut commands,
        &asset_server,
        q_board,
        q_scale_factor.single().0,
    );
}

pub fn despawn_game_over_menu(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = q_main_menu.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    q_board: Query<&Board>,
    scale_fac: f32,
) -> Entity {
    let winner_text = match q_board.single().board_won_by() {
        crate::board::WinState::WonByCross => "X won the game!",
        crate::board::WinState::WonByNought => "O won the game!",
        crate::board::WinState::Tie => "Tie nobody won!",
        crate::board::WinState::NotWon => unreachable!(),
    };

    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // === Menu Background
            parent
                .spawn((
                    NodeBundle {
                        style: get_menu_background_style(scale_fac),
                        background_color: Color::BLACK.with_a(0.6).into(),
                        ..Default::default()
                    },
                    UiScale,
                ))
                .with_children(|parent| {
                    // === Title ===
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                winner_text,
                                get_title_text_style(asset_server, scale_fac),
                            ),
                            ..Default::default()
                        },
                        TextScale,
                    ));
                    // === Play Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(scale_fac),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..Default::default()
                            },
                            PlayButton,
                            UiScale,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "Play again",
                                        get_button_text_style(asset_server, scale_fac),
                                    ),
                                    ..Default::default()
                                },
                                TextScale,
                            ));
                        });
                    // === Quit Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(scale_fac),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..Default::default()
                            },
                            QuitButton,
                            UiScale,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "Quit",
                                        get_button_text_style(asset_server, scale_fac),
                                    ),
                                    ..Default::default()
                                },
                                TextScale,
                            ));
                        });
                });
        })
        .id();

    main_menu_entity
}
