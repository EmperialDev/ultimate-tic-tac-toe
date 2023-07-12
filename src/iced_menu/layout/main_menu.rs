use bevy_iced::{
    iced::{
        alignment,
        theme::{Button, Container},
        widget::{button, container, text, Column},
        Length, Padding,
    },
    IcedContext,
};

use crate::iced_menu::{
    style::{MenuButton, MenuContainer},
    UiMessage,
};

pub fn main_menu(mut ctx: IcedContext<UiMessage>) {
    let title = text("Ultimate Tic Tac Toe");

    let play_button = button(text("Play").horizontal_alignment(alignment::Horizontal::Center))
        .width(Length::Fixed(80.0))
        .style(Button::Custom(Box::new(MenuButton::Play)))
        .on_press(UiMessage::PlayButton);

    let quit_button = button(text("Quit").horizontal_alignment(alignment::Horizontal::Center))
        .width(Length::Fixed(80.0))
        .style(Button::Custom(Box::new(MenuButton::Quit)))
        .on_press(UiMessage::QuitButton);

    let menu = Column::new()
        .push(title)
        .push(play_button)
        .push(quit_button)
        .spacing(5)
        .align_items(bevy_iced::iced::Alignment::Center);

    let manu_container = container(menu)
        .style(Container::Custom(Box::new(MenuContainer)))
        .padding(Padding::new(4.0));

    let content = container(manu_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

    ctx.display(content);
}
