use crate::component;
use crate::core;
use crate::profile;
use crate::theme;
use crate::widget::bold;
use iced::alignment;
use iced::widget::column;
use iced::widget::{button, container, image, row, text, vertical_space};
use iced::Element;
use iced::Length;

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    SummonerFetched(Result<core::Summoner, core::summoner::RequestError>),
}

fn summoner_icon<'a>(icon: Option<image::Handle>, level: u32) -> Element<'a, Message> {
    let image: Element<Message> = if let Some(handle) = icon {
        image(handle).into()
    } else {
        vertical_space().height(96).into()
    };

    component::Modal::new(
        container(image)
            .width(96.0)
            .height(96.0)
            .center_x()
            .center_y()
            .padding(2.0)
            .style(theme::summoner_icon_container()),
        container(bold(level).size(10))
            .padding([1, 4, 2, 4]) // TODO: fix this alignment issue (text doesnt seem to get centered)
            .center_y()
            .style(theme::summoner_level_container()),
    )
    .horizontal_alignment(iced::Alignment::Center)
    .vertical_alignment(iced::Alignment::End)
    .into()
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdateProfile(String),
}

pub struct Summoner {
    summoner_name: String,
    riot_id: Option<core::RiotId>,
    level: u32,
    icon: u32,
    icon_image: Option<image::Handle>,
}

impl Summoner {
    pub fn from_profile(profile: &profile::Data) -> Self {
        let riot_id = profile
            .games
            .first()
            .map(|game| game.participant(profile.summoner.puuid()).unwrap().riot_id);
        let summoner_name = profile.summoner.name().to_string();
        let level = profile.summoner.level();
        let icon = profile.summoner.icon_id() as u32;
        let path = format!(
            "{}{}.png",
            concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\profileicon\\"),
            icon
        );
        let icon_image = Some(iced::widget::image::Handle::from_path(path));

        Self {
            summoner_name,
            riot_id,
            level,
            icon,
            icon_image,
        }
    }

    pub fn new(icon: u32) -> Self {
        Summoner {
            summoner_name: String::from("Summoner"),
            riot_id: None,
            level: 111,
            icon,
            icon_image: None,
        }
    }

    pub fn load_icon(&mut self) {
        let path = format!(
            "{}{}.png",
            concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\img\\profileicon\\"),
            self.icon
        );
        self.icon_image = Some(iced::widget::image::Handle::from_path(path));
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::Update => Some(Event::UpdateProfile(self.summoner_name.clone())),
            Message::SummonerFetched(Ok(summoner)) => {
                self.summoner_name = summoner.name().to_string();
                self.level = summoner.level();
                self.icon = summoner.icon_id() as u32;
                self.load_icon();

                None
            }
            Message::SummonerFetched(Err(error)) => {
                tracing::error!("{error:?}");

                None
            }
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
                            .style(theme::sub_text())
                    ]
                    .spacing(8)
                    .align_items(iced::Alignment::Center)
                    .into(),
                    Some(
                        text(format!("Prev. {}", &self.summoner_name))
                            .style(theme::sub_text())
                            .size(12)
                            .into(),
                    ),
                ),
                None => (
                    row![
                        text(&self.summoner_name).size(24),
                        text(format!("#{}", riot_id.tagline))
                            .size(20)
                            .style(theme::gray_text())
                    ]
                    .spacing(2)
                    .align_items(iced::Alignment::Center)
                    .into(),
                    None,
                ),
            },
            None => (text(&self.summoner_name).size(24).into(), None),
        };

        let update_button = button("Update")
            .style(theme::update_button())
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
                .padding([8, 0, 8, 0]),
        )
        .center_x()
        .width(Length::Fill)
        .style(theme::timeline_container())
        .into()
    }
}
