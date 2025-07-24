use crate::core::account;
use crate::profile;
use crate::theme;

use iced::Element;
use iced::Length;
use iced::alignment;
use iced::widget::column;
use iced::widget::stack;
use iced::widget::{button, container, image, row, text};

#[derive(Debug, Clone)]
pub enum Message {
    Update,
}

fn icon<'a>(icon: image::Handle, level: u32) -> Element<'a, Message> {
    stack![
        container(image(icon))
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
    riot_id: account::RiotId,
    level: u32,
    icon: image::Handle,
}

impl Summoner {
    pub fn from_profile(profile: &profile::Data) -> Self {
        let riot_id = profile.summoner.account.riot_id.clone();
        let level = profile.summoner.level as u32;
        let icon = image::Handle::from_bytes(profile.icon.clone());

        Self {
            riot_id,
            level,
            icon,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Update => Some(Event::UpdateProfile(self.riot_id.to_string())),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon = icon(self.icon.clone(), self.level);

        let name = self.riot_id.name.as_deref().unwrap_or("missing");
        let tagline = self.riot_id.tagline.as_deref().unwrap_or("name");

        let name = row![
            text(name).size(24),
            text!("#{tagline}").size(24).style(theme::text)
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let update = button("Update")
            .style(theme::update)
            .on_press(Message::Update);

        let inner = column![
            name,
            container(update)
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
