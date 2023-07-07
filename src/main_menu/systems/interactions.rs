use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::components::*;
use crate::main_menu::styles::{PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

pub fn interact_with_play_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

pub fn interact_with_quit_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}