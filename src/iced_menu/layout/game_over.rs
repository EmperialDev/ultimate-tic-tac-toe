use std::sync::Mutex;

use bevy::prelude::{Component, Query};
use bevy_iced::{
    iced::{
        alignment,
        theme::Container,
        widget::{container, text, Column, Row},
        Alignment, Color, Element, Length, Padding, Point,
    },
    iced_wgpu::Renderer,
    IcedContext,
};
use iced::widget::{canvas, Canvas};

use crate::{
    board::CrossOrNought,
    iced_menu::{
        components::menu_button,
        menu_shapes::{generate_cross_path, generate_nought_path},
        style::{MenuButtonStyle, MenuContainer},
        UiMessage,
    },
    CROSS_COLOR, NOUGHT_COLOR,
};

pub fn game_over_menu(mut ctx: IcedContext<UiMessage>, symbol: Query<&Symbol>) {
    let title: Element<'_, UiMessage, Renderer> = if let Ok(symbol) = symbol.get_single() {
        let symbol_canvas = Canvas::new(symbol).height(20).width(20);

        let title = text("Won the game").vertical_alignment(alignment::Vertical::Center);

        Row::new()
            .push(symbol_canvas)
            .push(title)
            .spacing(2)
            .align_items(Alignment::Center)
            .into()
    } else {
        Column::new()
            .push(text("Tie"))
            .push(text("No one won"))
            .align_items(Alignment::Center)
            .into()
    };

    let play_again_button = menu_button("Play again", MenuButtonStyle::Play, UiMessage::PlayButton);

    let quit_button = menu_button("Quit", MenuButtonStyle::Quit, UiMessage::QuitButton);

    let menu = Column::new()
        .push(title)
        .push(play_again_button)
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

#[derive(Debug, Component)]
pub struct Symbol {
    symbol_cache: Mutex<canvas::Cache>,
    winner: CrossOrNought,
}

impl Symbol {
    pub fn cross() -> Self {
        Self {
            symbol_cache: Default::default(),
            winner: CrossOrNought::Cross,
        }
    }

    pub fn nought() -> Self {
        Self {
            symbol_cache: Default::default(),
            winner: CrossOrNought::Nought,
        }
    }
}

impl<Message> canvas::Program<Message> for Symbol {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let symbol = self
            .symbol_cache
            .lock()
            .unwrap()
            .draw(bounds.size(), |frame| {
                let (path, color) = match self.winner {
                    CrossOrNought::Cross => (
                        generate_cross_path(Point::new(10.0, 10.0), 18.0, 3.0),
                        Color::from_rgb(CROSS_COLOR.r(), CROSS_COLOR.g(), CROSS_COLOR.b()),
                    ),
                    CrossOrNought::Nought => (
                        generate_nought_path(Point::new(10.0, 10.0), 18.0, 3.0),
                        Color::from_rgb(NOUGHT_COLOR.r(), NOUGHT_COLOR.g(), NOUGHT_COLOR.b()),
                    ),
                };

                frame.fill(&path, color);
            });
        vec![symbol]
    }
}
