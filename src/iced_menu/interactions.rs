use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::UiMessage;

pub fn interaction_system(
    mut messages: EventReader<UiMessage>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for msg in messages.iter() {
        match msg {
            UiMessage::PlayButton => app_state_next_state.set(AppState::Game),
            UiMessage::QuitButton => app_exit_event_writer.send(AppExit),
        }
    }
}
