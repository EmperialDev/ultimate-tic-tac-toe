use bevy_iced::{
    iced::{
        alignment, theme,
        widget::{button, text, Button},
        Length,
    },
    iced_wgpu::Renderer,
};

use super::{style::MenuButtonStyle, POPPINS_MEDIUM};

pub fn menu_button<T>(
    button_text: &str,
    style: MenuButtonStyle,
    message: T,
) -> Button<'_, T, Renderer> {
    button(
        text(button_text)
            .size(16)
            .font(POPPINS_MEDIUM)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .width(Length::Fixed(72.0))
    .style(theme::Button::Custom(Box::new(style)))
    .on_press(message)
}
