use iced::{
    widget::{button, container, horizontal_space, pick_list, row, text_input},
    Alignment, Element, Length,
};

use crate::core::Region;
use crate::theme;
use crate::theme::icon;

#[derive(Clone, Debug)]
pub enum Message {
    TextChanged(String),
    SearchPressed,
    RegionSelected(Region),
}

pub enum Event {
    SearchRequested { riot_id: String, region: Region },
}

#[derive(Debug, Clone)]
pub struct SearchBar {
    text: String,
    region: Region,
}

impl SearchBar {
    pub fn new() -> SearchBar {
        SearchBar {
            text: String::new(),
            region: Region::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::TextChanged(text) => {
                self.text = text;
                None
            }
            Message::SearchPressed => Some(Event::SearchRequested {
                riot_id: self.text.clone(),
                region: self.region,
            }),
            Message::RegionSelected(region) => {
                self.region = region;

                None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let search = container(icon::search().width(16.0).height(16.0));

        row![
            container(
                row![
                    text_input("Search for a summoner or champion", &self.text)
                        .on_input(Message::TextChanged)
                        .on_submit(Message::SearchPressed)
                        .style(theme::search_text_input)
                        .size(12),
                    button(search)
                        .style(theme::search)
                        .on_press(Message::SearchPressed),
                ]
                .align_y(Alignment::Center)
            )
            .style(theme::search_bar)
            .width(Length::FillPortion(4)),
            horizontal_space().width(4),
            pick_list(Region::iter(), Some(self.region), Message::RegionSelected)
                .font(theme::EXTRA_BOLD)
                .text_size(12)
                .padding(5)
                .width(Length::Shrink)
                .style(theme::region)
                .menu_style(theme::region_menu),
        ]
        .align_y(Alignment::Center)
        .into()
    }
}
