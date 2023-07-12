use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin};
use bevy_iced::{iced::Font, IcedSettings};

use crate::AppState;

use self::{
    interactions::interaction_system,
    layout::{game_over::game_over_menu, main_menu::main_menu},
};

pub mod components;
pub mod interactions;
pub mod layout;
pub mod style;

pub struct IcedMenuPlugin;

const POPPINS_SEMI_BOLD: Font = Font::External {
    name: "Poppins-SemiBold",
    bytes: include_bytes!("..\\..\\assets\\fonts\\Poppins-SemiBold.ttf"),
};
const POPPINS_MEDIUM: Font = Font::External {
    name: "Poppins-Medium",
    bytes: include_bytes!("..\\..\\assets\\fonts\\Poppins-Medium.ttf"),
};

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
