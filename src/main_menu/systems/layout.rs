use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = q_main_menu.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
                .spawn(NodeBundle {
                    style: MENU_BACKGROUND_STYLE,
                    background_color: Color::BLACK.with_a(0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // === Title ===
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Ultimate Tic Tac Toe",
                            get_title_text_style(asset_server),
                        ),
                        ..Default::default()
                    });
                    // === Play Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..Default::default()
                            },
                            PlayButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Play",
                                    get_button_text_style(asset_server),
                                ),
                                ..Default::default()
                            });
                        });
                    // === Quit Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..Default::default()
                            },
                            QuitButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Quit",
                                    get_button_text_style(asset_server),
                                ),
                                ..Default::default()
                            });
                        });
                });
        })
        .id();

    main_menu_entity
}
