use bevy_iced::{
    iced::{
        theme::Container,
        widget::{container, text, Column},
        Length, Padding,
    },
    IcedContext,
};

use crate::iced_menu::{
    components::menu_button,
    style::{MenuButtonStyle, MenuContainer},
    UiMessage, POPPINS_SEMI_BOLD,
};

pub fn main_menu(mut ctx: IcedContext<UiMessage>) {
    let title = text("Ultimate Tic Tac Toe").font(POPPINS_SEMI_BOLD);

    let play_button = menu_button("Play", MenuButtonStyle::Play, UiMessage::PlayButton);

    let quit_button = menu_button("Quit", MenuButtonStyle::Quit, UiMessage::QuitButton);

    let menu = Column::new()
        .push(title)
        .push(play_button)
        .push(quit_button)
        .spacing(4)
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
