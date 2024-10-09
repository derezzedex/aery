use crate::core;
use crate::theme;
use iced::widget::{container, Space};
use iced::Length;

pub fn bold<'a>(text: impl ToString) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text.to_string()).font(iced::Font {
        weight: iced::font::Weight::Semibold,
        ..theme::ROBOTO_NORMAL
    })
}

pub fn left_border<'a, Message: 'a>(
    result: core::game::Result,
) -> iced::widget::Container<'a, Message> {
    container(Space::new(6.0, 0.0))
        .style(move |_| theme::left_border(result))
        .height(Length::Fill)
}

pub fn small_text<'a>(text: impl ToString) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text.to_string())
        .color(theme::SUB_TEXT)
        .size(8.0)
}
