use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_,  FontAssets>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Poppins-Light.ttf")]
    pub poppins_light: Handle<Font>,
    #[asset(path = "fonts/Poppins-Medium.ttf")]
    pub poppins_medium: Handle<Font>,
    #[asset(path = "fonts/Poppins-SemiBold.ttf")]
    pub poppins_semi_bold: Handle<Font>,

}