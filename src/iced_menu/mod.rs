use bevy::prelude::{
    App, Commands, Entity, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin,
    Query, With,
};
use bevy_iced::{iced::Font, IcedSettings};

use crate::{
    board::{Board, WinState},
    AppState,
};

use self::{
    interactions::interaction_system,
    layout::{
        game_over::{game_over_menu, Symbol},
        main_menu::main_menu,
    },
};

pub mod components;
pub mod interactions;
pub mod layout;
mod menu_shapes;
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
            // Store the symbol
            .add_system(spawn_canvas_symbol.in_schedule(OnEnter(AppState::GameOver)))
            // Game Over Menu
            .add_systems((game_over_menu, interaction_system).in_set(OnUpdate(AppState::GameOver)));
    }
}

fn spawn_canvas_symbol(
    mut commands: Commands,
    board: Query<&Board>,
    symbol: Query<Entity, With<Symbol>>,
) {
    for entity in &symbol {
        commands.entity(entity).despawn();
    }

    match board.single().board_won_by() {
        WinState::WonByCross => {
            commands.spawn(Symbol::cross());
        }
        WinState::WonByNought => {
            commands.spawn(Symbol::nought());
        }
        WinState::NotWon | WinState::Tie => (),
    };
}

#[derive(Clone)]
pub enum UiMessage {
    PlayButton,
    QuitButton,
}
