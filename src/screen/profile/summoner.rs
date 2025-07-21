use crate::core::account;
use crate::profile;
use crate::theme;

use iced::Element;
use iced::Length;
use iced::alignment;
use iced::widget::column;
use iced::widget::stack;
use iced::widget::{button, container, image, row, text, vertical_space};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
}

fn summoner_icon<'a>(icon: Option<image::Handle>, level: u32) -> Element<'a, Message> {
    let image: Element<Message> = if let Some(handle) = icon {
        image(handle).into()
    } else {
        vertical_space().height(96).into()
    };

    stack![
        container(image)
            .center_x(96.0)
            .center_y(96.0)
            .padding(2.0)
            .style(theme::summoner_icon),
        container(
            container(text(level).font(theme::EXTRA_BOLD).size(12))
                .padding([2, 4])
                .style(theme::summoner_level),
        )
        .align_bottom(Length::Fill)
        .center_x(Length::Fill)
    ]
    .into()
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdateProfile(String),
}

#[derive(Debug, Clone)]
pub struct Summoner {
    summoner_name: String,
    riot_id: account::RiotId,
    level: u32,
    icon_image: Option<image::Handle>,
}

impl Summoner {
    pub fn from_profile(profile: &profile::Data) -> Self {
        let riot_id = profile.summoner.account.riot_id.clone();
        let summoner_name = profile.summoner.name().to_string();
        let level = profile.summoner.level as u32;
        let icon_image = Some(image::Handle::from_bytes(profile.icon.clone()));

        Self {
            summoner_name,
            riot_id,
            level,
            icon_image,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Update => Some(Event::UpdateProfile(self.summoner_name.clone())),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let icon = summoner_icon(self.icon_image.clone(), self.level);

        let name = self.riot_id.name.clone().unwrap_or(String::from("Unknown"));

        let name = row![text(name).size(24),]
            .push_maybe(
                self.riot_id
                    .tagline
                    .as_ref()
                    .map(|tagline| text(format!("#{}", tagline)).size(24).style(theme::text)),
            )
            .spacing(8)
            .align_y(iced::Alignment::Center);

        let update_button = button("Update")
            .style(theme::update)
            .on_press(Message::Update);

        let inner = column![
            name,
            container(update_button)
                .height(48)
                .align_y(alignment::Vertical::Bottom)
        ];

        // TODO: display ladder rank and past season ranks

        container(
            column![row![icon, inner.spacing(1)].spacing(16)]
                .spacing(8)
                .padding([8, 0]),
        )
        .width(Length::Fill)
        .style(theme::timeline)
        .into()
    }
}
