mod menu;
mod pick_list;
use pick_list::PickList;

use crate::core;
use crate::theme;
use iced::Length;
use iced::widget::{Space, container};

pub fn left_border<'a, Message: 'a>(
    result: core::game::Result,
) -> iced::widget::Container<'a, Message> {
    container(Space::new(6.0, 0.0))
        .style(move |theme| theme::left_border(theme, result))
        .height(Length::Fill)
}

pub fn pick_list<'a, T, Message, Theme, Renderer>(
    options: &'a [T],
    selected: Option<&'a T>,
    view: impl Fn(&'a T) -> iced::Element<'a, Message, Theme, Renderer>,
) -> PickList<'a, T, Message, Theme, Renderer>
where
    T: PartialEq + Clone + 'a,
    Message: Clone + 'a,
    Theme: pick_list::Catalog + menu::Catalog + 'a,
    Renderer: iced::advanced::Renderer + iced_blur::Renderer + 'a,
{
    PickList::new(options, selected, view)
}
