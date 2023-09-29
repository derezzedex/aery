use crate::theme;
use iced::widget::{container, Space};
use iced::Length;

#[macro_export]
macro_rules! text {
    ($($arg:tt)*) => {
        iced::widget::Text::new(format!($($arg)*))
    }
}

pub fn bold<'a>(text: impl ToString) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text.to_string()).font(iced::Font {
        weight: iced::font::Weight::Semibold,
        ..Default::default()
    })
}

pub fn left_border<'a, Message: 'a>(win: bool) -> iced::widget::Container<'a, Message> {
    container(Space::new(6.0, 0.0))
        .style(theme::left_border_container(win))
        .height(Length::Fill)
}

pub fn small_text<'a>(text: impl ToString) -> iced::widget::Text<'a> {
    iced::widget::Text::new(text.to_string())
        .style(theme::sub_text())
        .size(8.0)
}

/// size 8
pub fn very_small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(8.0, 8.0))
        .style(theme::icon_container())
        .max_width(8.0)
        .max_height(8.0)
}

/// size 10
pub fn small_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(10.0, 10.0))
        .style(theme::icon_container())
        .max_width(10.0)
        .max_height(10.0)
}

/// size 12
pub fn medium_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(12.0, 12.0))
        .style(theme::icon_container())
        .max_width(12.0)
        .max_height(12.0)
}

/// size 18
pub fn medium_large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(18.0, 18.0))
        .style(theme::icon_container())
        .max_width(18.0)
        .max_height(18.0)
}

/// size 48
pub fn large_icon<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(48.0, 48.0))
        .style(theme::icon_container())
        .max_width(48.0)
        .max_height(48.0)
}
