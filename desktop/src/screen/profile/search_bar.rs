use iced::{
    widget::{button, container, horizontal_space, image, row, text, text_input, Space},
    Alignment, Element, Length,
};

use crate::theme;
use crate::theme::search_icon;

#[derive(Clone, Debug)]
pub enum Message {
    TextChanged(String),
    SearchPressed,
    RegionPressed,
}

pub enum Event {
    SearchRequested(String),
}

pub struct SearchBar {
    text: String,
}

fn logo<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
    container(Space::new(28.0, 28.0))
        .style(theme::icon_container())
        .max_width(28.0)
        .max_height(28.0)
}

impl SearchBar {
    pub fn new() -> SearchBar {
        SearchBar {
            text: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::TextChanged(text) => {
                self.text = text;
                None
            }
            Message::SearchPressed => Some(Event::SearchRequested(self.text.clone())),
            Message::RegionPressed => None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let region = "BR";

        let search = container(image(search_icon()).width(12.0).height(12.0)).padding(2);

        container(row![
            logo(),
            horizontal_space().width(Length::FillPortion(2)),
            container(
                row![
                    text_input("Search for a summoner or champion", &self.text)
                        .on_input(Message::TextChanged)
                        .on_submit(Message::SearchPressed)
                        .style(theme::search_bar_text_input())
                        .size(12),
                    button(text(region).size(10))
                        .width(Length::Shrink)
                        .padding([2, 4, 2, 4])
                        .style(theme::region_button())
                        .on_press(Message::RegionPressed),
                    button(search)
                        .style(iced::theme::Button::Text)
                        .on_press(Message::SearchPressed),
                ]
                .align_items(Alignment::Center)
            )
            .style(theme::search_bar_container())
            .width(Length::FillPortion(4)),
            horizontal_space().width(Length::FillPortion(2)),
        ])
        .padding(8)
        .style(theme::dark_container())
        .into()
    }
}
