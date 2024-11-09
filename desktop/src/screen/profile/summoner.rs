use crate::assets::Assets;
use crate::core::summoner;
use crate::profile;
use crate::theme;

use iced::alignment;
use iced::padding;
use iced::widget::column;
use iced::widget::stack;
use iced::widget::{button, container, image, row, text, vertical_space};
use iced::Element;
use iced::Length;

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
            container(text(level).font(theme::SEMIBOLD).size(10))
                .padding(padding::top(1).right(4).bottom(2).left(4)) // TODO: fix this alignment issue (text doesnt seem to get centered)
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

pub struct Summoner {
    summoner_name: String,
    riot_id: Option<summoner::RiotId>,
    level: u32,
    icon_image: Option<image::Handle>,
}

impl Summoner {
    pub fn from_profile(assets: &mut Assets, profile: &profile::Data) -> Self {
        let riot_id = profile
            .games
            .first()
            .map(|game| game.participant(profile.summoner.puuid()).unwrap().riot_id);
        let summoner_name = profile.summoner.name().to_string();
        let level = profile.summoner.level();
        let icon = profile.summoner.icon_id() as u32;
        let icon_image = Some(assets.get_summoner_icon(icon as usize));

        Self {
            summoner_name,
            riot_id,
            level,
            icon_image,
        }
    }

    pub fn new(_icon: u32) -> Self {
        Summoner {
            summoner_name: String::from("Summoner"),
            riot_id: None,
            level: 111,
            icon_image: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Update => Some(Event::UpdateProfile(self.summoner_name.clone())),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let icon = summoner_icon(self.icon_image.clone(), self.level);

        let (name, previously): (Element<_>, Option<Element<_>>) = match &self.riot_id {
            Some(riot_id) => match &riot_id.name {
                Some(riot_name) => (
                    row![
                        text(riot_name).size(24),
                        text(format!("#{}", riot_id.tagline))
                            .size(24)
                            .color(theme::SUB_TEXT)
                    ]
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                    .into(),
                    Some(
                        text(format!("Prev. {}", &self.summoner_name))
                            .color(theme::SUB_TEXT)
                            .size(12)
                            .into(),
                    ),
                ),
                None => (
                    row![
                        text(&self.summoner_name).size(24),
                        text(format!("#{}", riot_id.tagline))
                            .size(20)
                            .color(theme::GRAY_TEXT)
                    ]
                    .spacing(2)
                    .align_y(iced::Alignment::Center)
                    .into(),
                    None,
                ),
            },
            None => (text(&self.summoner_name).size(24).into(), None),
        };

        let update_button = button("Update")
            .style(theme::update)
            .on_press(Message::Update);

        let mut inner = column![name];

        if let Some(text) = previously {
            inner = inner.push(text);
        }

        inner = inner.push(
            container(update_button)
                .height(48)
                .align_y(alignment::Vertical::Bottom),
        );

        // TODO: display ladder rank and past season ranks

        container(
            column![row![icon, inner.spacing(1)].spacing(16)]
                .spacing(8)
                .width(Length::Fill)
                .max_width(920)
                .padding([8, 0]),
        )
        .center_x(Length::Fill)
        .style(theme::timeline)
        .into()
    }
}
