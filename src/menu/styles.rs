use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub fn get_menu_background_style(scale_fac: f32) -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        size: Size::new(Val::Auto, Val::Auto),
        gap: Size::new(Val::Px(8.0 * scale_fac), Val::Px(8.0 * scale_fac)),
        padding: UiRect::all(Val::Px(16.0 * scale_fac)),
        ..Style::DEFAULT
    }
}

pub fn get_button_style(scale_fac: f32) -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        size: Size::new(Val::Px(200.0 * scale_fac), Val::Px(80.0 * scale_fac)),
        ..Style::DEFAULT
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>, scale_fac: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Poppins-SemiBold.ttf"),
        font_size: 64.0 * scale_fac,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>, scale_fac: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Poppins-Medium.ttf"),
        font_size: 32.0 * scale_fac,
        color: Color::WHITE,
    }
}
