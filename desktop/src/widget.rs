use crate::core;
use crate::theme;
use iced::widget::{container, Space};
use iced::Length;

pub fn left_border<'a, Message: 'a>(
    result: core::game::Result,
) -> iced::widget::Container<'a, Message> {
    container(Space::new(6.0, 0.0))
        .style(move |theme| theme::left_border(theme, result))
        .height(Length::Fill)
}
