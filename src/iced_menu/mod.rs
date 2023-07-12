use bevy::prelude::*;
use bevy_iced::IcedSettings;

use crate::AppState;

use self::{
    interactions::interaction_system,
    layout::{game_over::game_over_menu, main_menu::main_menu},
};

pub mod interactions;
pub mod layout;
pub mod style;
pub mod components;

pub struct IcedMenuPlugin;

impl Plugin for IcedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiMessage>()
            .insert_resource(IcedSettings {
                scale_factor: Some(1.0),
                ..Default::default()
            })
            // Main menu
            .add_systems((main_menu, interaction_system).in_set(OnUpdate(AppState::MainMenu)))
            // Game Over Menu
            .add_systems((game_over_menu, interaction_system).in_set(OnUpdate(AppState::GameOver)));
    }
}

#[derive(Clone)]
pub enum UiMessage {
    PlayButton,
    QuitButton,
}
