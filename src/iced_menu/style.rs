use bevy_iced::{
    iced::{
        self, color,
        widget::{button, container},
    },
    iced_wgpu::Theme,
};

pub struct MenuContainer;

impl iced::widget::container::StyleSheet for MenuContainer {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: Some(iced::Background::Color(color!(0, 0, 0, 0.6))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
        }
    }
}

pub enum MenuButtonStyle {
    Play,
    Quit,
}

impl iced::widget::button::StyleSheet for MenuButtonStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Color::from_rgb(0.15, 0.15, 0.15).into()),
            text_color: color!(250, 250, 250),
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active_style = &self.active(style);

        button::Appearance {
            background: Some(iced::Color::from_rgb(0.25, 0.25, 0.25).into()),
            border_radius: active_style.border_radius,
            text_color: active_style.text_color,
            shadow_offset: active_style.shadow_offset,
            border_width: active_style.border_width,
            border_color: active_style.border_color,
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active_style = &self.active(style);

        let pressed_color = match self {
            MenuButtonStyle::Play => iced::Color::from_rgb(0.35, 0.75, 0.35),
            MenuButtonStyle::Quit => iced::Color::from_rgb(0.9, 0.15, 0.15),
        };

        button::Appearance {
            background: Some(pressed_color.into()),
            border_radius: active_style.border_radius,
            text_color: active_style.text_color,
            shadow_offset: active_style.shadow_offset,
            border_width: active_style.border_width,
            border_color: active_style.border_color,
        }
    }
}
