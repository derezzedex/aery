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
