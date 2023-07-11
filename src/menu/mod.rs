pub mod components;
pub mod interactions;
pub mod layout;
pub mod styles;

use bevy::prelude::*;

use crate::AppState;

use self::{
    interactions::*,
    layout::{
        game_over::{despawn_game_over_menu, spawn_game_over_menu},
        main_menu::*,
    },
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter state system
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            // Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
