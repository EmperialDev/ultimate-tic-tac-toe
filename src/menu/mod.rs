pub mod layout;
pub mod components;
pub mod styles;
pub mod interactions;

use bevy::prelude::*;

use crate::AppState;

use self::{layout::{main_menu::*, game_over::{spawn_game_over_menu, despawn_game_over_menu}}, interactions::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter state system
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GaneOver)))
            // Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::GaneOver)),
            )
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GaneOver)));
    }
}